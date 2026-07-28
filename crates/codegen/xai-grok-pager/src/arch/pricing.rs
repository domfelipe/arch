//! Token cost estimation for Arch (USD per million tokens).

/// Per-model prices in **USD per 1M tokens**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelPrices {
    pub input_per_mtok: f64,
    pub output_per_mtok: f64,
    /// Cached input tokens (often cheaper). When `None`, cache billed as input.
    pub cache_read_per_mtok: Option<f64>,
}

impl ModelPrices {
    pub const fn new(input: f64, output: f64) -> Self {
        Self {
            input_per_mtok: input,
            output_per_mtok: output,
            cache_read_per_mtok: None,
        }
    }

    pub const fn with_cache(mut self, cache: f64) -> Self {
        self.cache_read_per_mtok = Some(cache);
        self
    }
}

/// Built-in Arch catalog prices (illustrative, fixed for tests).
pub fn prices_for_model(slug: &str) -> Option<ModelPrices> {
    let s = slug.to_ascii_lowercase();
    // Match slug families used by the router.
    if s.contains("mini") || s.contains("fast") {
        return Some(ModelPrices::new(0.30, 0.50).with_cache(0.075));
    }
    if s.contains("vision") {
        return Some(ModelPrices::new(2.00, 10.00).with_cache(0.50));
    }
    if s.contains("grok-4") || s.contains("deep") {
        return Some(ModelPrices::new(3.00, 15.00).with_cache(0.75));
    }
    if s.contains("grok-3") || s.contains("grok-build") {
        return Some(ModelPrices::new(1.00, 5.00).with_cache(0.25));
    }
    None
}

/// Estimate USD cost from usage counters. Returns `None` when prices are unknown.
pub fn estimate_cost_usd(
    input_tokens: u64,
    output_tokens: u64,
    cache_read_tokens: u64,
    prices: Option<ModelPrices>,
) -> Option<f64> {
    let p = prices?;
    let cache_rate = p.cache_read_per_mtok.unwrap_or(p.input_per_mtok);
    let cost = (input_tokens as f64 / 1_000_000.0) * p.input_per_mtok
        + (output_tokens as f64 / 1_000_000.0) * p.output_per_mtok
        + (cache_read_tokens as f64 / 1_000_000.0) * cache_rate;
    Some(cost)
}

pub fn format_cost_label(cost: Option<f64>) -> String {
    match cost {
        Some(c) if c < 0.01 => format!("${c:.4}"),
        Some(c) => format!("${c:.3}"),
        None => "no price".to_string(),
    }
}

/// Single-line status: `12.3K tok · $0.042` or `12.3K tok · no price`.
pub fn format_token_cost_line(total_tokens: u64, cost: Option<f64>) -> String {
    let tok = format_token_count(total_tokens);
    format!("{tok} · {}", format_cost_label(cost))
}

fn format_token_count(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M tok", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K tok", n as f64 / 1_000.0)
    } else {
        format!("{n} tok")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priced_path_matches_formula() {
        let prices = ModelPrices::new(1.0, 5.0).with_cache(0.25);
        // 1M input + 100k output + 200k cache
        let cost = estimate_cost_usd(1_000_000, 100_000, 200_000, Some(prices)).unwrap();
        let expected = 1.0 + 0.5 + 0.05; // 1.55
        assert!((cost - expected).abs() < 1e-9, "cost={cost}");
        assert_eq!(format_cost_label(Some(cost)), "$1.550");
    }

    #[test]
    fn missing_price_path() {
        assert!(estimate_cost_usd(100, 50, 0, None).is_none());
        assert_eq!(format_cost_label(None), "no price");
        assert_eq!(
            format_token_cost_line(12_300, None),
            "12.3K tok · no price"
        );
    }

    #[test]
    fn catalog_covers_router_slugs() {
        for slug in ["grok-3-mini", "grok-3", "grok-4", "grok-4.5", "grok-2-vision"] {
            assert!(
                prices_for_model(slug).is_some(),
                "missing price for {slug}"
            );
        }
        assert!(prices_for_model("totally-unknown-model-xyz").is_none());
    }

    #[test]
    fn cache_defaults_to_input_rate_when_unset() {
        let p = ModelPrices::new(2.0, 4.0);
        let cost = estimate_cost_usd(500_000, 0, 500_000, Some(p)).unwrap();
        assert!((cost - 2.0).abs() < 1e-9); // both halves at input rate
    }
}
