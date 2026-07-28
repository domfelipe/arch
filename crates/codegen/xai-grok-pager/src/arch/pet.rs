//! Global Arch pet chrome — compact monochrome **angel-only** sprites.
//!
//! Source: `assets/arch/angel-gemini.svg` with background fills stripped
//! (no checker / flat gray). Rendered above the prompt, right-aligned.

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

    /// Single-line glyph (short terminals / status fallback). No label text.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Idle => "▄██▄",
            Self::Working => "▄▀█▄",
            Self::Happy => "▄██*",
        }
    }

    /// Kept for API compatibility; chrome no longer shows labels.
    pub fn label(self) -> &'static str {
        ""
    }

    /// Multi-line half-block angel silhouette (angel paths only, downscaled).
    ///
    /// 5 rows × ~7 cols — fits the band above the prompt when right-aligned.
    pub fn sprite(self) -> &'static [&'static str] {
        match self {
            // halo · head · wings · body · feet
            Self::Idle => &[
                "  ▄▄▄  ",
                " █████ ",
                "▄█████▄",
                "▀█████▀",
                " ██▀▀  ",
            ],
            Self::Working => &[
                "  ▄▄▄  ",
                " █▀█▀█ ",
                "▄█████▄",
                "▀█████▀",
                " ██▀▀  ",
            ],
            Self::Happy => &[
                " *▄▄▄* ",
                " █████ ",
                "▄█████▄",
                "▀█████▀",
                " ██▀▀  ",
            ],
        }
    }

    /// Preferred pet band height (rows).
    pub const BAND_HEIGHT: u16 = 5;
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
    if prev == PetState::Happy {
        return PetState::Idle;
    }
    PetState::Idle
}

/// Single-line chrome — glyph only (no "angel" text).
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
    fn chrome_is_glyph_only_no_label_text() {
        let line = format_pet_chrome(PetState::Idle);
        assert_eq!(line, PetState::Idle.glyph());
        assert!(!line.to_ascii_lowercase().contains("angel"));
        assert!(!line.contains("busy"));
        assert!(!line.contains("yay"));
        assert!(line.contains('▄') || line.contains('█'));
    }

    #[test]
    fn sprite_is_compact_half_blocks() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            assert_eq!(rows.len(), PetState::BAND_HEIGHT as usize, "{s:?}");
            for row in rows {
                assert!(
                    row.chars()
                        .any(|c| matches!(c, '█' | '▄' | '▀' | ' ' | '*')),
                    "{s:?} unexpected chars: {row:?}"
                );
                assert!(row.chars().count() <= 8, "sprite too wide: {row:?}");
            }
        }
    }

    #[test]
    fn glyphs_differ_by_state() {
        assert_ne!(PetState::Idle.glyph(), PetState::Working.glyph());
        assert_ne!(PetState::Idle.glyph(), PetState::Happy.glyph());
    }
}
