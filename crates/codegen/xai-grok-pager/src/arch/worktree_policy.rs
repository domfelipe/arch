//! Arch task-tab worktree policy helpers (testable without full TUI).

use crate::app::app_view::WorktreeMode;

/// Whether a new Arch task should create an isolated git worktree.
///
/// `mode` is the resolved UI preference; `in_git_repo` comes from the cwd.
/// Returns true only when the repo is git and mode is Always (or Ask resolved yes).
pub fn should_auto_worktree(mode: WorktreeMode, in_git_repo: bool) -> bool {
    if !in_git_repo {
        return false;
    }
    matches!(mode, WorktreeMode::Always)
}

/// Documented skip reasons for logs/toasts.
pub fn skip_reason(mode: WorktreeMode, in_git_repo: bool) -> Option<&'static str> {
    if !in_git_repo {
        return Some("not a git repository");
    }
    match mode {
        WorktreeMode::Never => Some("worktrees disabled (new_session_worktree_mode=never)"),
        WorktreeMode::Ask => Some("worktree deferred to user prompt"),
        WorktreeMode::Always => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::TempDir;

    #[test]
    fn auto_worktree_only_in_git_with_always() {
        assert!(!should_auto_worktree(WorktreeMode::Always, false));
        assert!(!should_auto_worktree(WorktreeMode::Never, true));
        assert!(!should_auto_worktree(WorktreeMode::Ask, true));
        assert!(should_auto_worktree(WorktreeMode::Always, true));
    }

    #[test]
    fn temp_git_repo_is_detected_and_non_git_skipped() {
        let git = TempDir::new().unwrap();
        let status = Command::new("git")
            .args(["init"])
            .current_dir(git.path())
            .status()
            .expect("git init");
        assert!(status.success());
        let in_git = git.path().join(".git").exists();
        assert!(in_git);
        assert!(should_auto_worktree(WorktreeMode::Always, in_git));
        assert_eq!(skip_reason(WorktreeMode::Always, in_git), None);

        let plain = TempDir::new().unwrap();
        let not_git = plain.path().join(".git").exists();
        assert!(!not_git);
        assert!(!should_auto_worktree(WorktreeMode::Always, not_git));
        assert_eq!(
            skip_reason(WorktreeMode::Always, not_git),
            Some("not a git repository")
        );
    }

    #[test]
    fn never_mode_documents_skip() {
        assert_eq!(
            skip_reason(WorktreeMode::Never, true),
            Some("worktrees disabled (new_session_worktree_mode=never)")
        );
    }
}
