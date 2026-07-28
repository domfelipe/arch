//! Global Arch pet chrome — monochrome braille **angel** sprites.
//!
//! Source silhouette: `assets/arch/angel-gemini.svg` (Gemini angel).
//! Status bar uses a 1-line glyph; multi-line sprites available for tests.

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

    /// Compact braille angel for the status bar (halo + wings + body).
    ///
    /// Extracted/downscaled from the Gemini angel SVG silhouette.
    pub fn glyph(self) -> &'static str {
        match self {
            // calm: open wings
            Self::Idle => "⣿⢟⣼",
            // focused: tighter center
            Self::Working => "⣿⢹⣏",
            // celebrate: denser wing tips
            Self::Happy => "⣿⣿⣧",
        }
    }

    /// Short label after the glyph.
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "angel",
            Self::Working => "busy",
            Self::Happy => "yay",
        }
    }

    /// Multi-line braille angel (downscaled from SVG silhouette).
    pub fn sprite(self) -> &'static [&'static str] {
        match self {
            Self::Idle => &[
                "⣿⣿⢟⣼",
                "⣿⢹⣏⣿",
                "⣿⣿⣧⣿",
            ],
            Self::Working => &[
                "⣿⣿⢿⣵",
                "⣽⢸⣏⣯",
                "⢿⣿⠧⡿",
            ],
            Self::Happy => &[
                "⣾⣿⢿⣽",
                "⣿⢸⣏⣿",
                "⢿⣿⣷⡿",
            ],
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
    if prev == PetState::Happy {
        return PetState::Idle;
    }
    PetState::Idle
}

/// Single-line chrome: `⣿⢟⣼ angel`.
pub fn format_pet_chrome(state: PetState) -> String {
    format!("{} {}", state.glyph(), state.label())
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
    fn chrome_is_braille_angel() {
        let line = format_pet_chrome(PetState::Idle);
        assert!(line.contains(PetState::Idle.glyph()));
        assert!(line.contains("angel"));
        // Braille block range U+2800–U+28FF
        assert!(
            line.chars().any(|c| ('\u{2800}'..='\u{28ff}').contains(&c)),
            "expected braille in {line}"
        );
    }

    #[test]
    fn sprite_rows_are_braille() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            assert!(rows.len() >= 3, "{s:?}");
            for row in rows {
                assert!(
                    row.chars().any(|c| ('\u{2800}'..='\u{28ff}').contains(&c)),
                    "{s:?} row not braille: {row}"
                );
            }
        }
    }

    #[test]
    fn glyphs_differ_by_state() {
        assert_ne!(PetState::Idle.glyph(), PetState::Working.glyph());
        assert_ne!(PetState::Idle.glyph(), PetState::Happy.glyph());
    }
}
