//! `/split` — toggle Arch two-pane task view.

use crate::app::actions::Action;
use crate::slash::command::{CommandExecCtx, CommandResult, SlashCommand};

pub struct SplitCommand;

impl SlashCommand for SplitCommand {
    fn name(&self) -> &str {
        "split"
    }

    fn aliases(&self) -> &[&str] {
        &[]
    }

    fn description(&self) -> &str {
        "Toggle side-by-side view of two task tabs"
    }

    fn usage(&self) -> &str {
        "/split"
    }

    fn run(&self, _ctx: &mut CommandExecCtx, _args: &str) -> CommandResult {
        CommandResult::Action(Action::ToggleSplitView)
    }
}
