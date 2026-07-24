//! Arch multi-task tabs, split view, and agent handoffs.
//!
//! Thin layer on top of the existing multi-agent map (`AppView::agents`):
//! - Ctrl+T / `/new` already creates a parallel agent (see `ActionId::NewSession`).
//! - Tab cycle switches `active_view` among open agents.
//! - Split keeps a secondary agent visible beside the active one.
//! - Handoff opens a related tab with a target soul (`developer` / `qa`).

use crate::app::actions::{Action, Effect};
use crate::app::agent::AgentId;
use crate::app::app_view::{ActiveView, AppView};
use crate::app::dispatch::ctx::{SwitchCause, switch_to_agent};
use crate::app::dispatch::session::lifecycle::dispatch_new_session_inner_with_id;
use crate::views::session_title;

/// Cycle focus to the next open agent tab (wraps).
pub(in crate::app::dispatch) fn dispatch_next_agent_tab(app: &mut AppView) -> Vec<Effect> {
    cycle_agent_tab(app, 1)
}

/// Cycle focus to the previous open agent tab (wraps).
pub(in crate::app::dispatch) fn dispatch_prev_agent_tab(app: &mut AppView) -> Vec<Effect> {
    cycle_agent_tab(app, -1)
}

fn cycle_agent_tab(app: &mut AppView, delta: isize) -> Vec<Effect> {
    if app.agents.len() < 2 {
        app.show_toast("Only one task tab open — Ctrl+T for a new one");
        return vec![];
    }
    let ActiveView::Agent(current) = app.active_view else {
        // Prefer the first agent if not already on one.
        if let Some((&first, _)) = app.agents.first() {
            switch_to_agent(app, first, SwitchCause::Picker);
        }
        return vec![];
    };
    let ids: Vec<AgentId> = app.agents.keys().copied().collect();
    let Some(pos) = ids.iter().position(|id| *id == current) else {
        return vec![];
    };
    let len = ids.len() as isize;
    let next = ((pos as isize + delta).rem_euclid(len)) as usize;
    let target = ids[next];
    if target != current {
        switch_to_agent(app, target, SwitchCause::Picker);
    }
    vec![]
}

/// Toggle split view: active agent + another open agent side by side.
pub(in crate::app::dispatch) fn dispatch_toggle_split(app: &mut AppView) -> Vec<Effect> {
    if app.split_secondary.is_some() {
        app.split_secondary = None;
        app.show_toast("Split view off");
        return vec![];
    }
    if app.agents.len() < 2 {
        app.show_toast("Need two task tabs to split — Ctrl+T to open another");
        return vec![];
    }
    let ActiveView::Agent(active) = app.active_view else {
        app.show_toast("Open a task first");
        return vec![];
    };
    // Prefer previous tab in order; else next.
    let ids: Vec<AgentId> = app.agents.keys().copied().collect();
    let Some(pos) = ids.iter().position(|id| *id == active) else {
        return vec![];
    };
    let peer = if pos > 0 {
        ids[pos - 1]
    } else {
        ids.get(1).copied().unwrap_or(active)
    };
    if peer == active {
        app.show_toast("Need two task tabs to split");
        return vec![];
    }
    app.split_secondary = Some(peer);
    app.show_toast("Split view on (Ctrl+Shift+S to exit)");
    vec![]
}

/// Hand off the current task to another Arch soul (`developer` / `qa`).
///
/// Creates a new parallel tab, stamps the target profile into the create-session
/// meta, labels the tab, and seeds a handoff brief as the first prompt.
pub(in crate::app::dispatch) fn dispatch_handoff(
    app: &mut AppView,
    target: &str,
    brief: Option<&str>,
) -> Vec<Effect> {
    let target = normalize_soul(target);
    if !matches!(target.as_str(), "developer" | "qa") {
        app.show_toast("Handoff target must be developer or qa");
        return vec![];
    }

    let ActiveView::Agent(source_id) = app.active_view else {
        app.show_toast("Open a task before handoff");
        return vec![];
    };

    let source_title = app
        .agents
        .get(&source_id)
        .map(session_title::entry_title)
        .unwrap_or_else(|| "task".to_string());
    let summary = brief
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| derive_handoff_summary(app, source_id));

    let source_label = app
        .agents
        .get(&source_id)
        .and_then(|a| a.arch_soul.clone())
        .unwrap_or_else(|| "agent".to_string());

    let seed = format!(
        "[Arch handoff] From **{source_label}** tab “{source_title}” → **{target}**.\n\n\
         ## Brief\n{summary}\n\n\
         Proceed as the {target} agent. Keep changes minimal and report clearly."
    );

    // Create parallel agent (does not tear down the source).
    let (new_id, mut effects) = dispatch_new_session_inner_with_id(app, None);

    if let Some(agent) = app.agents.get_mut(&new_id) {
        agent.display_name = Some(soul_display_name(&target));
        agent.arch_soul = Some(target.clone());
        agent.arch_related_to = Some(source_id);
    }
    if let Some(source) = app.agents.get_mut(&source_id) {
        source.arch_related_to = Some(new_id);
    }

    // Stamp agentProfile on the CreateSession effect for this handoff tab.
    for effect in &mut effects {
        if let Effect::CreateSession {
            agent_id,
            agent_profile,
            ..
        } = effect
            && *agent_id == new_id
        {
            *agent_profile = Some(target.clone());
        }
    }

    // Auto-split source + handoff target when possible.
    app.split_secondary = Some(source_id);
    switch_to_agent(app, new_id, SwitchCause::New);

    // Queue brief; drains when SessionCreated lands (same path as CLI initial prompt).
    effects.extend(crate::app::dispatch::dispatch(Action::SendPrompt(seed), app));

    app.show_toast(&format!("Handoff → {target} (related tab open)"));
    effects
}

fn normalize_soul(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}

fn soul_display_name(soul: &str) -> String {
    match soul {
        "developer" => "Developer".to_string(),
        "qa" => "QA".to_string(),
        other => other.to_string(),
    }
}

fn derive_handoff_summary(app: &AppView, source_id: AgentId) -> String {
    let Some(agent) = app.agents.get(&source_id) else {
        return "Review the related task context and continue.".to_string();
    };
    if let Some(line) = session_title::last_user_prompt_line(agent) {
        return format!("Latest user request:\n{line}");
    }
    format!(
        "Session “{}” has no user prompts yet — coordinate with the source agent.",
        session_title::entry_title(agent)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::dispatch::dispatch;
    use crate::app::dispatch::tests::test_app_with_agent;

    #[test]
    fn cycle_tabs_requires_two_agents() {
        let mut app = test_app_with_agent();
        let effects = dispatch_next_agent_tab(&mut app);
        assert!(effects.is_empty());
    }

    #[test]
    fn cycle_tabs_switches_active() {
        let mut app = test_app_with_agent();
        let first = match app.active_view {
            ActiveView::Agent(id) => id,
            _ => panic!("expected agent"),
        };
        // Spawn a second agent via NewSession.
        let _ = dispatch(Action::NewSession, &mut app);
        assert!(app.agents.len() >= 2);
        let second = match app.active_view {
            ActiveView::Agent(id) => id,
            _ => panic!("expected agent after new"),
        };
        assert_ne!(first, second);
        dispatch_prev_agent_tab(&mut app);
        assert_eq!(app.active_view, ActiveView::Agent(first));
        dispatch_next_agent_tab(&mut app);
        assert_eq!(app.active_view, ActiveView::Agent(second));
    }

    #[test]
    fn toggle_split_needs_two_then_clears() {
        let mut app = test_app_with_agent();
        let _ = dispatch_toggle_split(&mut app);
        assert!(app.split_secondary.is_none());
        let _ = dispatch(Action::NewSession, &mut app);
        let _ = dispatch_toggle_split(&mut app);
        assert!(app.split_secondary.is_some());
        let _ = dispatch_toggle_split(&mut app);
        assert!(app.split_secondary.is_none());
    }

    #[test]
    fn handoff_creates_related_tab() {
        let mut app = test_app_with_agent();
        let source = match app.active_view {
            ActiveView::Agent(id) => id,
            _ => panic!("expected agent"),
        };
        if let Some(a) = app.agents.get_mut(&source) {
            a.arch_soul = Some("developer".into());
            a.display_name = Some("Developer".into());
        }
        let effects = dispatch_handoff(&mut app, "qa", Some("Check the login bug"));
        assert!(
            effects
                .iter()
                .any(|e| matches!(e, Effect::CreateSession { agent_profile: Some(p), .. } if p == "qa")),
            "expected CreateSession with qa profile: {effects:?}"
        );
        assert!(app.agents.len() >= 2);
        let ActiveView::Agent(new_id) = app.active_view else {
            panic!("handoff should focus new tab");
        };
        assert_ne!(new_id, source);
        let new = &app.agents[&new_id];
        assert_eq!(new.arch_soul.as_deref(), Some("qa"));
        assert_eq!(new.display_name.as_deref(), Some("QA"));
        assert_eq!(new.arch_related_to, Some(source));
        assert_eq!(app.split_secondary, Some(source));
    }
}
