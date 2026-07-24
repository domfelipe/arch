//! Arch product surface: model routing, pricing, pet chrome.
//!
//! Pure helpers live here so unit tests do not need the full TUI.

pub mod model_router;
pub mod pet;
pub mod pricing;
pub mod worktree_policy;

pub use model_router::{
    ModelMode, ModelProfile, ReasoningChoice, RouteDecision, route, route_auto,
};
pub use pet::{PetState, pet_state_from_agent};
pub use pricing::{ModelPrices, estimate_cost_usd, format_cost_label, format_token_cost_line};
