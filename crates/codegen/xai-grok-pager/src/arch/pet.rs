//! Global Arch pet — monochrome half-block angel from the product sticker PNG.
//!
//! Source: `assets/arch/angel-pet.png` (transparent pixel art; no checker bg).
//! Rendered **right-aligned** above the prompt; **no text label**.

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

    /// Single-line fallback (short terminals). Glyph only — no label.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Idle => "▄█▄",
            Self::Working => "▄▀▄",
            Self::Happy => "*█*",
        }
    }

    /// Unused in UI (kept for API compatibility).
    pub fn label(self) -> &'static str {
        ""
    }

    /// Multi-line half-block silhouette derived from `angel-pet.png`
    /// (alpha crop + NEAREST downscale 18×20 → 10 half-block rows).
    ///
    /// Layout (top→bottom): halo · head · wings+shoulders · torso · feet.
    pub fn sprite(self) -> &'static [&'static str] {
        match self {
            Self::Idle => &[
                "        ▄█▄       ",
                "       ▄███▄      ",
                "    ███████▄█▄    ",
                "▄██████████▀█████▄",
                "██████████████████",
                "███████▀██████████",
                "█████████████▄████",
                " ██████████▀█████ ",
                " ▀██▀ ████▄█▀███▀ ",
                "      ████▀▀▀     ",
            ],
            // Eyes slightly closed / focused (middle face band).
            Self::Working => &[
                "        ▄█▄       ",
                "       ▄███▄      ",
                "    ███████▄█▄    ",
                "▄██████████▀█████▄",
                "██████████████████",
                "███████▄██████████",
                "█████████████▀████",
                " ██████████▀█████ ",
                " ▀██▀ ████▄█▀███▀ ",
                "      ████▀▀▀     ",
            ],
            // Halo sparkle.
            Self::Happy => &[
                "       *▄█▄*      ",
                "       ▄███▄      ",
                "    ███████▄█▄    ",
                "▄██████████▀█████▄",
                "██████████████████",
                "███████▀██████████",
                "█████████████▄████",
                " ██████████▀█████ ",
                " ▀██▀ ████▄█▀███▀ ",
                "      ████▀▀▀     ",
            ],
        }
    }

    /// Preferred pet band height (rows of half-blocks).
    pub const BAND_HEIGHT: u16 = 10;
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

/// Single-line chrome — glyph only (no “angel” text).
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
    }

    #[test]
    fn sprite_matches_sticker_grid() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            assert_eq!(rows.len(), PetState::BAND_HEIGHT as usize, "{s:?}");
            // Halo cross-ish top
            assert!(
                rows[0].contains('▄') || rows[0].contains('█') || rows[0].contains('*'),
                "halo missing: {:?}",
                rows[0]
            );
            // Wings wider than feet row
            let wing_w = rows[3].trim().chars().count();
            let feet_w = rows[9].trim().chars().count();
            assert!(wing_w >= feet_w, "wings should span body {s:?}");
            // No checker noise of pure solid block field
            let solid = rows.iter().filter(|r| r.chars().all(|c| c == '█')).count();
            assert!(solid < rows.len(), "sprite collapsed to solid blocks");
        }
    }

    #[test]
    fn glyphs_differ_by_state() {
        assert_ne!(PetState::Idle.glyph(), PetState::Working.glyph());
        assert_ne!(PetState::Idle.glyph(), PetState::Happy.glyph());
    }
}
