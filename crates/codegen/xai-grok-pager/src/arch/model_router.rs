//! Arch model selection: Auto / Profile / Exact with fast|balanced|deep|vision.

/// How the user (or UI) selected a model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelMode {
    /// Hybrid router picks profile + reasoning from the prompt/task text.
    Auto,
    /// Explicit profile (still maps to a model family + default reasoning).
    Profile(ModelProfile),
    /// Exact model slug (and optional reasoning left to caller).
    Exact {
        model: String,
        reasoning: Option<ReasoningChoice>,
    },
}

/// Named Arch model profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelProfile {
    Fast,
    Balanced,
    Deep,
    Vision,
}

impl ModelProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fast => "fast",
            Self::Balanced => "balanced",
            Self::Deep => "deep",
            Self::Vision => "vision",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "fast" => Some(Self::Fast),
            "balanced" | "default" => Some(Self::Balanced),
            "deep" | "thinking" => Some(Self::Deep),
            "vision" | "image" | "multimodal" => Some(Self::Vision),
            _ => None,
        }
    }

    /// Canonical model slug family for this profile (testable fixed map).
    /// Prefer resolving via [`resolve_profile_slug`] against the live catalog.
    pub fn model_slug(self) -> &'static str {
        match self {
            Self::Fast => "grok-3-mini",
            Self::Balanced => "grok-4",
            Self::Deep => "grok-4",
            Self::Vision => "grok-2-vision",
        }
    }

    /// Preferential id/name candidates for matching the live `/models` catalog.
    /// First hit wins (case-insensitive contains on id or display name).
    pub fn catalog_candidates(self) -> &'static [&'static str] {
        match self {
            Self::Fast => &[
                "grok-3-mini",
                "grok-mini",
                "mini",
                "fast",
                "code-fast",
            ],
            Self::Balanced => &["grok-4", "grok-4.5", "grok-3", "grok", "balanced"],
            Self::Deep => &[
                "grok-4.5",
                "grok-4",
                "heavy",
                "reasoning",
                "deep",
            ],
            Self::Vision => &["vision", "grok-2-vision", "image", "multimodal"],
        }
    }

    pub fn default_reasoning(self) -> ReasoningChoice {
        match self {
            Self::Fast => ReasoningChoice::Low,
            Self::Balanced => ReasoningChoice::Medium,
            Self::Deep => ReasoningChoice::High,
            Self::Vision => ReasoningChoice::Low,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReasoningChoice {
    Low,
    Medium,
    High,
    XHigh,
}

impl ReasoningChoice {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
        }
    }
}

/// Result of routing for UI + SwitchModel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDecision {
    pub profile: ModelProfile,
    pub model_slug: String,
    pub reasoning: ReasoningChoice,
    pub mode: &'static str,
}

/// Resolve the active selection mode into a concrete decision.
/// Exact and Profile win over Auto.
pub fn route(mode: &ModelMode, prompt: &str) -> RouteDecision {
    match mode {
        ModelMode::Exact { model, reasoning } => {
            let profile = ModelProfile::parse(model).unwrap_or(ModelProfile::Balanced);
            RouteDecision {
                profile,
                model_slug: model.clone(),
                reasoning: reasoning.unwrap_or(profile.default_reasoning()),
                mode: "exact",
            }
        }
        ModelMode::Profile(p) => RouteDecision {
            profile: *p,
            model_slug: p.model_slug().to_string(),
            reasoning: p.default_reasoning(),
            mode: "profile",
        },
        ModelMode::Auto => {
            let mut d = route_auto(prompt);
            d.mode = "auto";
            d
        }
    }
}

/// Pick the best catalog model id for a profile.
///
/// `catalog` entries are `(id, display_name)` pairs from the agent model list.
/// Falls back to [`ModelProfile::model_slug`] when the catalog is empty or no
/// candidate matches — callers should still surface a clear error if the
/// session catalog later rejects the id.
pub fn resolve_profile_slug<'a>(
    profile: ModelProfile,
    catalog: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> String {
    let catalog: Vec<(&str, &str)> = catalog.into_iter().collect();
    if catalog.is_empty() {
        return profile.model_slug().to_string();
    }
    for cand in profile.catalog_candidates() {
        let c = cand.to_ascii_lowercase();
        if let Some((id, _)) = catalog.iter().find(|(id, name)| {
            let id_l = id.to_ascii_lowercase();
            let name_l = name.to_ascii_lowercase();
            id_l == c
                || name_l == c
                || id_l.contains(&c)
                || name_l.contains(&c)
        }) {
            return (*id).to_string();
        }
    }
    // Last resort: keep canonical slug (may fail server-side; better than silent wrong).
    profile.model_slug().to_string()
}

/// Fixed, testable Auto routing rules over prompt/task text.
pub fn route_auto(prompt: &str) -> RouteDecision {
    let p = prompt.to_ascii_lowercase();
    let profile = if looks_like_vision(&p) {
        ModelProfile::Vision
    } else if looks_like_deep(&p) {
        ModelProfile::Deep
    } else if looks_like_fast(&p) {
        ModelProfile::Fast
    } else {
        ModelProfile::Balanced
    };
    RouteDecision {
        profile,
        model_slug: profile.model_slug().to_string(),
        reasoning: profile.default_reasoning(),
        mode: "auto",
    }
}

fn looks_like_vision(p: &str) -> bool {
    p.contains("screenshot")
        || p.contains("image")
        || p.contains("png")
        || p.contains("jpg")
        || p.contains("vision")
        || p.contains("ui mock")
        || p.contains("pixel art")
}

fn looks_like_deep(p: &str) -> bool {
    p.contains("architect")
        || p.contains("design a system")
        || p.contains("root cause")
        || p.contains("refactor the whole")
        || p.contains("security audit")
        || p.contains("trade-off")
        || p.contains("tradeoff")
        || p.contains("migrate the monorepo")
}

fn looks_like_fast(p: &str) -> bool {
    p.contains("quick")
        || p.contains("typo")
        || p.contains("rename")
        || p.contains("one-liner")
        || p.contains("one liner")
        || p.contains("fix the nits")
        || p.starts_with("fix ")
        || p.contains("lint error")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_corpus_maps_expected_profiles() {
        let cases: &[(&str, ModelProfile)] = &[
            ("fix the lint error in main.rs", ModelProfile::Fast),
            ("quick rename foo to bar", ModelProfile::Fast),
            ("implement the login form", ModelProfile::Balanced),
            ("design a system for multi-tenant auth", ModelProfile::Deep),
            ("root cause the race in the worker", ModelProfile::Deep),
            ("look at this screenshot of the bug", ModelProfile::Vision),
            ("generate pixel art for the HUD", ModelProfile::Vision),
        ];
        for (prompt, expected) in cases {
            let d = route_auto(prompt);
            assert_eq!(d.profile, *expected, "prompt={prompt:?}");
            assert_eq!(d.model_slug, expected.model_slug());
            assert_eq!(d.reasoning, expected.default_reasoning());
        }
    }

    #[test]
    fn profile_mode_ignores_prompt() {
        let d = route(
            &ModelMode::Profile(ModelProfile::Deep),
            "quick typo fix",
        );
        assert_eq!(d.profile, ModelProfile::Deep);
        assert_eq!(d.mode, "profile");
        assert_eq!(d.model_slug, "grok-4");
        assert_eq!(d.reasoning, ReasoningChoice::High);
    }

    #[test]
    fn exact_mode_wins_over_auto_heuristics() {
        let d = route(
            &ModelMode::Exact {
                model: "custom-slug-xyz".into(),
                reasoning: Some(ReasoningChoice::XHigh),
            },
            "look at this screenshot",
        );
        assert_eq!(d.mode, "exact");
        assert_eq!(d.model_slug, "custom-slug-xyz");
        assert_eq!(d.reasoning, ReasoningChoice::XHigh);
    }

    #[test]
    fn resolve_profile_slug_picks_catalog_match() {
        let catalog = [
            ("grok-3-mini-id", "Grok 3 Mini"),
            ("grok-4.5-id", "Grok 4.5"),
            ("vision-id", "Grok 2 Vision"),
        ];
        assert_eq!(
            resolve_profile_slug(ModelProfile::Fast, catalog),
            "grok-3-mini-id"
        );
        assert_eq!(
            resolve_profile_slug(ModelProfile::Deep, catalog),
            "grok-4.5-id"
        );
        assert_eq!(
            resolve_profile_slug(ModelProfile::Vision, catalog),
            "vision-id"
        );
    }

    #[test]
    fn resolve_profile_slug_empty_catalog_falls_back() {
        let empty: [(&str, &str); 0] = [];
        assert_eq!(
            resolve_profile_slug(ModelProfile::Balanced, empty),
            ModelProfile::Balanced.model_slug()
        );
    }

    #[test]
    fn auto_fallback_is_balanced() {
        let d = route(&ModelMode::Auto, "please help with the feature");
        assert_eq!(d.profile, ModelProfile::Balanced);
        assert_eq!(d.mode, "auto");
    }
}
