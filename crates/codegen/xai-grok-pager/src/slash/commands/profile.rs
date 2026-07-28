//! `/profile` and `/auto` — Arch model selection modes (Profile / Auto).

use crate::app::actions::Action;
use crate::arch::model_router::{ModelMode, ModelProfile, route};
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};
use agent_client_protocol as acp;
use xai_grok_shell::sampling::types::ReasoningEffort;

/// `/profile <fast|balanced|deep|vision>` — lock a named Arch model profile.
pub struct ProfileCommand;

impl SlashCommand for ProfileCommand {
    fn name(&self) -> &str {
        "profile"
    }

    fn aliases(&self) -> &[&str] {
        &[]
    }

    fn description(&self) -> &str {
        "Select Arch model profile: fast | balanced | deep | vision"
    }

    fn usage(&self) -> &str {
        "/profile <fast|balanced|deep|vision>"
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn args_required(&self) -> bool {
        true
    }

    fn run(&self, ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let Some(profile) = ModelProfile::parse(args) else {
            return CommandResult::Error(
                "Usage: /profile <fast|balanced|deep|vision>\n\
                 Tip: use /model for any model in the live catalog (Ctrl+M)."
                    .into(),
            );
        };
        let decision = route(&ModelMode::Profile(profile), "");
        let model_id = resolve_profile_model_id(ctx, profile, &decision.model_slug);
        let effort = effort_from_reasoning(decision.reasoning);
        CommandResult::Action(Action::SwitchModel {
            model_id,
            effort,
        })
    }
}

/// `/auto [prompt hint]` — re-enable Auto routing (optionally from a hint).
pub struct AutoModelCommand;

impl SlashCommand for AutoModelCommand {
    fn name(&self) -> &str {
        "auto-model"
    }

    fn aliases(&self) -> &[&str] {
        &["automodel"]
    }

    fn description(&self) -> &str {
        "Route model automatically (fast/balanced/deep/vision) from a hint"
    }

    fn usage(&self) -> &str {
        "/auto-model [task hint]"
    }

    fn takes_args(&self) -> bool {
        true
    }

    fn run(&self, ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let hint = if args.trim().is_empty() {
            "implement the feature"
        } else {
            args.trim()
        };
        let decision = route(&ModelMode::Auto, hint);
        let model_id = resolve_profile_model_id(ctx, decision.profile, &decision.model_slug);
        let effort = effort_from_reasoning(decision.reasoning);
        CommandResult::Action(Action::SwitchModel {
            model_id,
            effort,
        })
    }
}

/// Map a profile decision onto a catalog `ModelId` when the session has models.
fn resolve_profile_model_id(
    ctx: &CommandExecCtx<'_>,
    profile: ModelProfile,
    fallback_slug: &str,
) -> acp::ModelId {
    use crate::arch::model_router::resolve_profile_slug;

    if !ctx.models.available.is_empty() {
        let catalog = ctx
            .models
            .available
            .iter()
            .map(|(id, info)| (id.0.as_ref(), info.name.as_str()));
        let slug = resolve_profile_slug(profile, catalog);
        if let Some(id) = ctx.models.resolve_by_name_or_id(&slug) {
            return id;
        }
        // Fuzzy already returned an id string from catalog — trust it.
        return acp::ModelId::new(std::sync::Arc::from(slug.as_str()));
    }
    // Catalog not loaded yet: keep canonical slug (same as pre-catalog /model).
    acp::ModelId::new(std::sync::Arc::from(fallback_slug))
}

fn effort_from_reasoning(
    r: crate::arch::model_router::ReasoningChoice,
) -> Option<ReasoningEffort> {
    use crate::arch::model_router::ReasoningChoice::*;
    Some(match r {
        Low => ReasoningEffort::Low,
        Medium => ReasoningEffort::Medium,
        High => ReasoningEffort::High,
        XHigh => ReasoningEffort::Xhigh,
    })
}
