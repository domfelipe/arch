//! Arch theme — deep indigo night with gold halo accents (angel product).
//!
//! Survives 256-color quantization better than pure TokyoNight blues by
//! keeping near-neutral backgrounds with saturated gold/violet accents.

use ratatui::style::{Color, Modifier};

use super::tokyonight::Theme;

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb(r, g, b)
}

#[allow(dead_code)]
mod palette {
    use super::*;

    // Deep indigo night (not pure black — slight blue cast)
    pub const BG: Color = rgb(12, 14, 22); // #0c0e16
    pub const BG_DARK: Color = rgb(10, 12, 18); // #0a0c12
    pub const BG_BASE: Color = rgb(18, 20, 32); // #121420
    pub const BG_HIGHLIGHT: Color = rgb(32, 36, 54); // #202436
    pub const BG_HOVER: Color = rgb(40, 44, 66); // #282c42

    // Text
    pub const FG: Color = rgb(236, 232, 248); // #ece8f8
    pub const FG_DIM: Color = rgb(180, 176, 210); // #b4b0d2
    pub const GRAY_DIM: Color = rgb(70, 72, 96); // #464860
    pub const GRAY: Color = rgb(110, 112, 140); // #6e708c
    pub const GRAY_BRIGHT: Color = rgb(150, 148, 180); // #9694b4

    // Angel gold + violet
    pub const GOLD: Color = rgb(232, 196, 120); // #e8c478
    pub const GOLD_SOFT: Color = rgb(255, 219, 141); // #ffdb8d
    pub const VIOLET: Color = rgb(170, 140, 255); // #aa8cff
    pub const VIOLET_DIM: Color = rgb(120, 100, 200); // #7864c8
    pub const CYAN: Color = rgb(120, 210, 230); // #78d2e6
    pub const GREEN: Color = rgb(140, 210, 160); // #8cd2a0
    pub const RED: Color = rgb(240, 120, 140); // #f0788c
    pub const ORANGE: Color = rgb(240, 170, 110); // #f0aa6e
    pub const TEAL: Color = rgb(90, 200, 180); // #5ac8b4

    pub const RED_DARK: Color = rgb(50, 16, 28);
    pub const GREEN_DARK: Color = rgb(12, 42, 28);
}
use palette::*;

impl Theme {
    /// Arch product theme — indigo night + gold accents.
    pub const fn arch() -> Self {
        Self {
            bg_base: BG_BASE,
            bg_light: BG_HIGHLIGHT,
            bg_dark: rgb(24, 26, 40),
            bg_highlight: BG_HIGHLIGHT,
            bg_hover: BG_HOVER,
            bg_terminal: BG,

            accent_user: GOLD_SOFT,
            accent_assistant: VIOLET,
            accent_thinking: VIOLET_DIM,
            accent_tool: GRAY_BRIGHT,
            accent_system: CYAN,
            accent_error: RED,
            accent_success: GREEN,
            accent_running: GOLD,
            accent_skill: VIOLET,

            text_primary: FG,
            text_secondary: FG_DIM,

            gray_dim: GRAY_DIM,
            gray: GRAY,
            gray_bright: GRAY_BRIGHT,

            command: GOLD,
            path: ORANGE,
            running: CYAN,
            warning: GOLD_SOFT,

            fuzzy_accent: VIOLET,

            accent_plan: GOLD_SOFT,
            accent_verify: VIOLET,
            accent_feedback: TEAL,
            accent_remember: GREEN,

            selection_border: rgb(70, 72, 100),
            prompt_border: rgb(50, 52, 74),
            prompt_border_active: GOLD,
            hover_border: rgb(36, 38, 56),

            accent_model: TEAL,

            scrollbar_bg: BG_DARK,
            scrollbar_fg: BG_HIGHLIGHT,

            diff_delete_bg: RED_DARK,
            diff_delete_fg: RED,
            diff_insert_bg: GREEN_DARK,
            diff_insert_fg: GREEN,
            diff_equal_fg: GRAY,
            diff_gutter_fg: GRAY,

            bg_visual: rgb(48, 50, 72),

            paste_bg: BG_DARK,
            paste_fg: FG_DIM,
            paste_dim: GRAY_DIM,

            md_heading_h1: GOLD,
            md_heading_h1_mod: Modifier::BOLD,
            md_heading_h2: VIOLET,
            md_heading_h2_mod: Modifier::BOLD,
            md_heading_h3: CYAN,
            md_heading_h3_mod: Modifier::BOLD,
            md_heading_h4: GRAY_BRIGHT,
            md_heading_h4_mod: Modifier::BOLD,
            md_heading_h5: GRAY,
            md_heading_h5_mod: Modifier::BOLD,
            md_heading_h6: GRAY_DIM,
            md_heading_h6_mod: Modifier::empty(),
            md_code: CYAN,
            md_task_checked: GREEN,
            md_task_unchecked: FG_DIM,
            md_muted: GRAY,
            md_code_bg: rgb(24, 26, 40),
            md_text: FG_DIM,
            link_fg: CYAN,
        }
    }
}
