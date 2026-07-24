//! `/handoff <developer|qa> [brief]` — open a related Arch agent tab.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

pub struct HandoffCommand;

impl SlashCommand for HandoffCommand {
    fn name(&self) -> &str {
        "handoff"
    }

    fn aliases(&self) -> &[&str] {
        &["ha"]
    }

    fn description(&self) -> &str {
        "Hand off this task to Developer or QA (new related tab)"
    }

    fn usage(&self) -> &str {
        "/handoff <developer|qa> [brief]"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, args: &str) -> CommandResult {
        let mut parts = args.splitn(2, char::is_whitespace);
        let target = parts.next().unwrap_or("").trim();
        if target.is_empty() {
            return CommandResult::Error(
                "Usage: /handoff <developer|qa> [brief]".into(),
            );
        }
        let brief = parts
            .next()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        CommandResult::Action(Action::Handoff {
            target: target.to_string(),
            brief,
        })
    }
}
