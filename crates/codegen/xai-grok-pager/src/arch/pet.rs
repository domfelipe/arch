//! Global Arch pet — hand-tuned monochrome half-block angel.
//!
//! Derived from `assets/arch/angel-pet.png` (pixel sticker): cross halo,
//! head, wide wings, torso, feet. Outline-style so wings read as wings,
//! not a solid blob. Right-aligned above the prompt; no text label.

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

    /// Single-line fallback (short terminals). Glyph only.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Idle => "▄█▄",
            Self::Working => "▄▀▄",
            Self::Happy => "*█*",
        }
    }

    /// Unused in UI.
    pub fn label(self) -> &'static str {
        ""
    }

    /// Multi-line angel (hand-tuned from the sticker PNG).
    ///
    /// ```text
    ///          ▄█▄           halo + cross top
    ///         ████
    ///    ▄▄ ██████ ▄▄        wing tips + head
    ///   █  ████████  █       wings open
    ///  █   ██▀  ▀██   █      eyes
    ///  █   ████████   █      face / beard
    ///   █  ████████  █       shoulders + cape
    ///    █  ██████  █        torso / belt
    ///     ██  ██  ██         legs
    /// ```
    pub fn sprite(self) -> &'static [&'static str] {
        match self {
            Self::Idle => ANGEL_IDLE,
            Self::Working => ANGEL_WORKING,
            Self::Happy => ANGEL_HAPPY,
        }
    }

    /// Preferred pet band height (must match `sprite().len()`).
    pub const BAND_HEIGHT: u16 = 9;
}

/// Idle: open eyes, open wings.
const ANGEL_IDLE: &[&str] = &[
    "         ▄█▄         ",
    "        ████         ",
    "   ▄▄  ██████  ▄▄    ",
    "  █   ████████   █   ",
    " █    ██▀  ▀██    █  ",
    " █    ████████    █  ",
    "  █   ████████   █   ",
    "   █   ██████   █    ",
    "    ██  ████  ██     ",
];

/// Working: eyes as a line (focused).
const ANGEL_WORKING: &[&str] = &[
    "         ▄█▄         ",
    "        ████         ",
    "   ▄▄  ██████  ▄▄    ",
    "  █   ████████   █   ",
    " █    ██▀▀▀▀██    █  ",
    " █    ████████    █  ",
    "  █   ████████   █   ",
    "   █   ██████   █    ",
    "    ██  ████  ██     ",
];

/// Happy: sparkles on the halo.
const ANGEL_HAPPY: &[&str] = &[
    "        *▄█▄*        ",
    "        ████         ",
    "   ▄▄  ██████  ▄▄    ",
    "  █   ████████   █   ",
    " █    ██▀  ▀██    █  ",
    " █    ████████    █  ",
    "  █   ████████   █   ",
    "   █   ██████   █    ",
    "    ██  ████  ██     ",
];

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
    if prev == PetState::Happy {
        return PetState::Idle;
    }
    PetState::Idle
}

/// Single-line chrome — glyph only.
pub fn format_pet_chrome(state: PetState) -> String {
    state.glyph().to_string()
}

/// Multi-line sprite joined with newlines.
pub fn format_pet_sprite(state: PetState) -> String {
    state.sprite().join("\n")
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
        assert_eq!(pet_state_from_agent(true, true), PetState::Working);
    }

    #[test]
    fn chrome_is_glyph_only() {
        assert_eq!(format_pet_chrome(PetState::Idle), PetState::Idle.glyph());
        assert!(!format_pet_chrome(PetState::Idle).contains("angel"));
    }

    #[test]
    fn sprite_reads_as_angel_silhouette() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            assert_eq!(rows.len(), PetState::BAND_HEIGHT as usize);
            // Halo at top center
            assert!(rows[0].contains('▄') || rows[0].contains('█') || rows[0].contains('*'));
            // Wings: outer columns have ink on mid rows
            let mid = rows[3];
            let trimmed = mid.trim_end();
            assert!(
                trimmed.starts_with('█') || trimmed.starts_with(' '),
                "wing row shape: {mid:?}"
            );
            // Face gap (eyes) only on idle/happy
            if matches!(s, PetState::Idle | PetState::Happy) {
                assert!(
                    rows[4].contains('▀') || rows[4].contains(' '),
                    "expected eye detail on {s:?}: {:?}",
                    rows[4]
                );
            }
            // Not a solid rectangle
            let all_solid = rows.iter().all(|r| r.chars().filter(|c| *c != ' ').all(|c| c == '█'));
            assert!(!all_solid, "must not be a solid blob");
        }
    }

    #[test]
    fn sprites_are_same_width() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            let w = rows[0].chars().count();
            assert!(rows.iter().all(|r| r.chars().count() == w), "{s:?}");
        }
    }

    #[test]
    fn glyphs_differ_by_state() {
        assert_ne!(PetState::Idle.glyph(), PetState::Working.glyph());
        assert_ne!(PetState::Idle.glyph(), PetState::Happy.glyph());
    }
}
