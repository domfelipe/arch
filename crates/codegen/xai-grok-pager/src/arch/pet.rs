//! Global Arch pet — braille **wings** matching the welcome logo art.
//!
//! Same style as `assets/logo/logo05.txt` (compact welcome wings), right-aligned
//! above the prompt. No text label.

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

    /// Single-line fallback — wing tips from the logo style.
    pub fn glyph(self) -> &'static str {
        match self {
            Self::Idle => "⠰⣶⣿⣶⠆",
            Self::Working => "⠰⣿⣿⣿⠆",
            Self::Happy => "⠰⣶*⣶⠆",
        }
    }

    /// Unused in UI.
    pub fn label(self) -> &'static str {
        ""
    }

    /// Multi-line wings (same braille language as the welcome logo).
    pub fn sprite(self) -> &'static [&'static str] {
        match self {
            Self::Idle => WINGS_IDLE,
            Self::Working => WINGS_WORKING,
            Self::Happy => WINGS_HAPPY,
        }
    }

    /// Matches `sprite().len()` — compact welcome wing art (logo05).
    pub const BAND_HEIGHT: u16 = 10;
}

// Compact wing pair from `logo05.txt` (welcome). Fixed width 49.
const WINGS_IDLE: &[&str] = &[
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣶⠆         ",
    "⠀⠀⠀⠀⠀⠀⠀⣠⢿⣷⠼⠓⠻⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⠛⠢⢼⡻⣆⡀      ",
    "⠀⠀⠀⠀⢀⣾⡟⠁⡄⢸⠻⢀⠰⣄⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡏⢀⠎⣀⠟⡇⢲⠀⠻⡷⣄    ",
    "⠀⠀⣰⠃⡼⠀⣿⡎⠀⣿⡆⡉⢾⣆⣈⠳⠹⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠟⠿⣁⣸⡿⢛⢶⣿⠀⢸⢿⠀⢳⠘⣧  ",
    "⢠⣿⠞⡟⠀⢸⠀⡇⠀⡟⣿⣷⡇⡄⣌⠙⠻⢿⣷⣿⣲⣤⣹⣦⡀⣰⣏⣡⠔⣫⣷⣿⠟⠋⣁⢠⢹⣷⣿⢻⠀⢸⠀⡇⠀⢹⠳⣽⡀",
    "⠀⠀⣸⠀⢸⠁⠸⣷⠀⢸⠀⢸⡈⠛⠛⠛⢿⣿⣿⣿⣾⣴⠸⣿⢵⣿⠏⡆⣷⣿⣿⢿⡿⠟⠛⠛⢁⡏⠀⡎⠀⣼⡇⠀⡇⠀⢷  ",
    "⠀⠸⣿⡾⡇⠀⠀⣿⣇⠀⢧⠀⠈⢧⣇⣏⠀⡟⢻⠇⢸⠷⠈⠁⠀⠈⠁⠸⡇⠸⣏⡿⡀⢹⣼⣾⠁⠀⡸⠀⢸⢻⠀⠀⢸⣷⣾  ",
    "⠀⠀⠀⠀⡇⠀⠀⣿⠀⢧⠀⢱⠀⠀⢹⡌⠀⠈⠙⣧⣼⠀⠀⠀⠀⠀⠀⠀⢷⣼⠃⠁⠀⢁⡟⠀⠀⡜⠀⣸⠀⢸⠀⠀⢸    ",
    "⠀⠀⠀⠀⢹⡆⠀⣿⠀⠀⢹⡀⠘⡄⠀⠈⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠀⢀⠇⢀⡎⠀⠀⢸⠀⠀⡇    ",
    "⠀⠀⠀⠀⠀⠸⣼⡏⢧⠀⠀⠈⢧⠀⢳⡀⠈⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃⠀⡼⠁⡸⠁⠀⠀⡸⢋⣧⠏     ",
];

// Same wings, slightly denser mid-band (working pulse).
const WINGS_WORKING: &[&str] = &[
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣶⠆         ",
    "⠀⠀⠀⠀⠀⠀⠀⣠⢿⣷⠼⠓⠻⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⠛⠢⢼⡻⣆⡀      ",
    "⠀⠀⠀⠀⢀⣾⡟⠁⡄⢸⠻⢀⠰⣄⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡏⢀⠎⣀⠟⡇⢲⠀⠻⡷⣄    ",
    "⠀⠀⣰⠃⡼⠀⣿⡎⠀⣿⡆⡉⢾⣆⣈⠳⠹⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠟⠿⣁⣸⡿⢛⢶⣿⠀⢸⢿⠀⢳⠘⣧  ",
    "⢠⣿⠞⡟⠀⢸⣿⡇⠀⡟⣿⣷⡇⡄⣌⠙⠻⢿⣷⣿⣲⣤⣹⣦⡀⣰⣏⣡⠔⣫⣷⣿⠟⠋⣁⢠⢹⣷⣿⢻⠀⢸⣿⡇⠀⢹⠳⣽⡀",
    "⠀⠀⣸⠀⢸⠁⠸⣷⠀⢸⠀⢸⡈⠛⠛⠛⢿⣿⣿⣿⣾⣴⠸⣿⢵⣿⠏⡆⣷⣿⣿⢿⡿⠟⠛⠛⢁⡏⠀⡎⠀⣼⡇⠀⡇⠀⢷  ",
    "⠀⠸⣿⡾⡇⠀⠀⣿⣇⠀⢧⠀⠈⢧⣇⣏⠀⡟⢻⠇⢸⠷⠈⠁⠀⠈⠁⠸⡇⠸⣏⡿⡀⢹⣼⣾⠁⠀⡸⠀⢸⢻⠀⠀⢸⣷⣾  ",
    "⠀⠀⠀⠀⡇⠀⠀⣿⠀⢧⠀⢱⠀⠀⢹⡌⠀⠈⠙⣧⣼⠀⠀⠀⠀⠀⠀⠀⢷⣼⠃⠁⠀⢁⡟⠀⠀⡜⠀⣸⠀⢸⠀⠀⢸    ",
    "⠀⠀⠀⠀⢹⡆⠀⣿⠀⠀⢹⡀⠘⡄⠀⠈⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠀⢀⠇⢀⡎⠀⠀⢸⠀⠀⡇    ",
    "⠀⠀⠀⠀⠀⠸⣼⡏⢧⠀⠀⠈⢧⠀⢳⡀⠈⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃⠀⡼⠁⡸⠁⠀⠀⡸⢋⣧⠏     ",
];

// Wing-tip sparkles.
const WINGS_HAPPY: &[&str] = &[
    "⠀⠀⠀⠀⠀⠀⠀⠀*⠰⣶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣶⠆*        ",
    "⠀⠀⠀⠀⠀⠀⠀⣠⢿⣷⠼⠓⠻⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⠟⠛⠢⢼⡻⣆⡀      ",
    "⠀⠀⠀⠀⢀⣾⡟⠁⡄⢸⠻⢀⠰⣄⢹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡏⢀⠎⣀⠟⡇⢲⠀⠻⡷⣄    ",
    "⠀⠀⣰⠃⡼⠀⣿⡎⠀⣿⡆⡉⢾⣆⣈⠳⠹⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⠟⠿⣁⣸⡿⢛⢶⣿⠀⢸⢿⠀⢳⠘⣧  ",
    "⢠⣿⠞⡟⠀⢸⠀⡇⠀⡟⣿⣷⡇⡄⣌⠙⠻⢿⣷⣿⣲⣤⣹⣦⡀⣰⣏⣡⠔⣫⣷⣿⠟⠋⣁⢠⢹⣷⣿⢻⠀⢸⠀⡇⠀⢹⠳⣽⡀",
    "⠀⠀⣸⠀⢸⠁⠸⣷⠀⢸⠀⢸⡈⠛⠛⠛⢿⣿⣿⣿⣾⣴⠸⣿⢵⣿⠏⡆⣷⣿⣿⢿⡿⠟⠛⠛⢁⡏⠀⡎⠀⣼⡇⠀⡇⠀⢷  ",
    "⠀⠸⣿⡾⡇⠀⠀⣿⣇⠀⢧⠀⠈⢧⣇⣏⠀⡟⢻⠇⢸⠷⠈⠁⠀⠈⠁⠸⡇⠸⣏⡿⡀⢹⣼⣾⠁⠀⡸⠀⢸⢻⠀⠀⢸⣷⣾  ",
    "⠀⠀⠀⠀⡇⠀⠀⣿⠀⢧⠀⢱⠀⠀⢹⡌⠀⠈⠙⣧⣼⠀⠀⠀⠀⠀⠀⠀⢷⣼⠃⠁⠀⢁⡟⠀⠀⡜⠀⣸⠀⢸⠀⠀⢸    ",
    "⠀⠀⠀⠀⢹⡆⠀⣿⠀⠀⢹⡀⠘⡄⠀⠈⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠃⠀⢀⠇⢀⡎⠀⠀⢸⠀⠀⡇    ",
    "⠀⠀⠀⠀⠀⠸⣼⡏⢧⠀⠀⠈⢧⠀⢳⡀⠈⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠃⠀⡼⠁⡸⠁⠀⠀⡸⢋⣧⠏     ",
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
    fn sprite_matches_welcome_wing_style() {
        for s in [PetState::Idle, PetState::Working, PetState::Happy] {
            let rows = s.sprite();
            assert_eq!(rows.len(), PetState::BAND_HEIGHT as usize, "{s:?}");
            // Braille present (same family as welcome logo)
            assert!(
                rows.iter().any(|r| r.chars().any(|c| ('\u{2800}'..='\u{28ff}').contains(&c))),
                "{s:?} missing braille"
            );
            // Wing pair: both left and right density
            let mid = rows[4];
            assert!(mid.contains('⣿') || mid.contains('⣷'), "dense mid wing: {mid}");
        }
    }

    #[test]
    fn glyphs_differ_by_state() {
        assert_ne!(PetState::Idle.glyph(), PetState::Working.glyph());
        assert_ne!(PetState::Idle.glyph(), PetState::Happy.glyph());
    }
}
