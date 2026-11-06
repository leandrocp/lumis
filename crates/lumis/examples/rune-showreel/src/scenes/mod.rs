pub(crate) mod blitz;
pub(crate) mod burn;
pub(crate) mod ignite;
pub(crate) mod langs;
pub(crate) mod review;
pub(crate) mod slots;

use rune::El;

/// The code most shots are about.
pub(crate) const PRICING: &str = "/// Totals a cart in the shopper's currency.
pub fn total_price(cart: &Cart, rates: &Rates) -> f64 {
    let mut total = 0.0;
    for item in &cart.items {
        let rate = rates.lookup(&item.region);
        for _ in 0..item.qty {
            total += item.price * rate.factor();
        }
    }
    let discount = cart.coupon.map(|c| c.percent);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_cart_is_free() {
        assert_eq!(total_price(&Cart::default(), &Rates::eu()), 0.0);
    }
}";

/// [`PRICING`] without its tests.
pub(crate) fn function() -> &'static str {
    &PRICING[..PRICING.find("\n\n#[cfg").unwrap_or(PRICING.len())]
}

pub(crate) const FILES: &[&str] = &[
    "src",
    "  cart.rs",
    "  pricing.rs",
    "  rates.rs",
    "  main.rs",
    "tests",
    "  checkout.rs",
    "Cargo.toml",
    "README.md",
];

/// Every shot, with its length, back to back.
/// A shot's view at its own time.
type Shot = fn(f64) -> El;

const SHOTS: &[(f64, Shot)] = &[
    (ignite::LEN, ignite::view),
    (slots::LEN, slots::view),
    (review::LEN, review::view),
    (burn::LEN, burn::view),
    (blitz::LEN, blitz::view),
    (langs::LEN, langs::view),
];

pub(crate) fn reel(mut t: f64) -> El {
    for &(len, view) in SHOTS {
        if t < len {
            return view(t);
        }
        t -= len;
    }
    let (len, view) = SHOTS[SHOTS.len() - 1];
    view(t + len)
}
