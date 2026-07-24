//! Compact Arch task-tab strip for multi-agent sessions.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use crate::app::agent::AgentId;
use crate::app::agent_view::AgentView;
use crate::theme::Theme;
use crate::views::session_title;
use indexmap::IndexMap;

/// Height reserved for the tab strip (one row).
pub const TAB_STRIP_HEIGHT: u16 = 1;

/// Paint a single-row tab strip. Returns the strip rect for hit-testing if needed.
pub fn render_tab_strip(
    buf: &mut Buffer,
    area: Rect,
    agents: &IndexMap<AgentId, AgentView>,
    active: AgentId,
    theme: &Theme,
) -> Rect {
    if area.height == 0 || area.width == 0 || agents.is_empty() {
        return Rect::default();
    }
    let strip = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: TAB_STRIP_HEIGHT.min(area.height),
    };

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::styled(
        " tabs ",
        Style::default().fg(theme.gray).add_modifier(Modifier::DIM),
    ));

    for (i, (id, agent)) in agents.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(
                " │ ",
                Style::default().fg(theme.gray).add_modifier(Modifier::DIM),
            ));
        }
        let mut label = if let Some(soul) = agent.arch_soul.as_deref() {
            format!("{}:{}", soul, short_title(agent))
        } else {
            short_title(agent)
        };
        if agent.arch_related_to.is_some() {
            label.push('↔');
        }
        // Soft cap so many tabs still fit.
        if label.chars().count() > 24 {
            label = label.chars().take(23).collect::<String>() + "…";
        }
        let active_style = if *id == active {
            Style::default()
                .fg(theme.text_primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.gray)
        };
        let marker = if *id == active { "● " } else { "○ " };
        spans.push(Span::styled(format!("{marker}{label}"), active_style));
    }

    Paragraph::new(Line::from(spans)).render(strip, buf);
    strip
}

fn short_title(agent: &AgentView) -> String {
    let title = session_title::entry_title(agent);
    if title.chars().count() > 18 {
        title.chars().take(17).collect::<String>() + "…"
    } else {
        title
    }
}
