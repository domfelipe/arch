//! Global Arch pixel-art angel pet state machine.

/// Activity-driven pet states for the always-on chrome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PetState {
    /// No active turn.
    Idle,
    /// Agent turn running / tools executing.
    Working,
    /// Turn just completed successfully (brief pulse).
    Happy,
}

impl PetState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Working => "working",
            Self::Happy => "happy",
        }
    }

    /// Compact 16-bit-ish braille glyph for the status chrome.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Idle => "ʚĭɞ",
            Self::Working => "ʚ˙ɞ",
            Self::Happy => "ʚ♥ɞ",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "angel",
            Self::Working => "angel!",
            Self::Happy => "angel♥",
        }
    }
}

/// Map session activity flags → pet state (pure, testable).
pub fn pet_state_from_agent(turn_running: bool, just_completed: bool) -> PetState {
    if turn_running {
        PetState::Working
    } else if just_completed {
        PetState::Happy
    } else {
        PetState::Idle
    }
}

/// Transition helper used by the TUI tick path.
pub fn next_pet_state(prev: PetState, turn_running: bool, turn_just_ended: bool) -> PetState {
    if turn_running {
        return PetState::Working;
    }
    if turn_just_ended {
        return PetState::Happy;
    }
    // Happy is sticky for one idle frame then settles to Idle.
    if prev == PetState::Happy {
        return PetState::Idle;
    }
    PetState::Idle
}

/// Single-line pet chrome: `ʚĭɞ angel`.
pub fn format_pet_chrome(state: PetState) -> String {
    format!("{} {}", state.glyph(), state.label())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_to_working_to_happy_to_idle() {
        let mut s = PetState::Idle;
        s = next_pet_state(s, true, false);
        assert_eq!(s, PetState::Working);
        s = next_pet_state(s, false, true);
        assert_eq!(s, PetState::Happy);
        s = next_pet_state(s, false, false);
        assert_eq!(s, PetState::Idle);
    }

    #[test]
    fn working_beats_happy_flag() {
        assert_eq!(
            pet_state_from_agent(true, true),
            PetState::Working
        );
    }

    #[test]
    fn chrome_includes_glyph() {
        let line = format_pet_chrome(PetState::Idle);
        assert!(line.contains(PetState::Idle.glyph()));
        assert!(line.contains("angel"));
    }
}
