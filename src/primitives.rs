#[cfg(any(feature = "minimal", feature = "address"))]
use rand::seq::IndexedRandom as _;
#[cfg(any(
    feature = "minimal",
    feature = "address",
    feature = "finance",
    feature = "personal"
))]
use rand::{Rng as _, RngCore};

#[cfg(feature = "minimal")]
pub fn lower_char<Rng: RngCore>(rng: &mut Rng) -> char {
    rng.random_range('a'..='z')
}

#[cfg(any(
    feature = "minimal",
    feature = "address",
    feature = "finance",
    feature = "personal"
))]
pub fn capital_char<Rng: RngCore>(rng: &mut Rng) -> char {
    rng.random_range('A'..='Z')
}

#[cfg(feature = "minimal")]
#[allow(clippy::unwrap_used, reason = "non empty string")]
pub fn alphanumeric_char<Rng: RngCore>(rng: &mut Rng) -> char {
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .choose(rng)
        .unwrap() as char
}

#[cfg(any(feature = "minimal", feature = "address"))]
#[allow(clippy::unwrap_used, reason = "non empty string")]
pub fn alphanumeric_capital_char<Rng: RngCore>(rng: &mut Rng) -> char {
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".choose(rng).unwrap() as char
}

#[cfg(feature = "minimal")]
pub fn boolean<Rng: RngCore>(rng: &mut Rng) -> &str {
    if rng.random_bool(0.5) {
        "True"
    } else {
        "False"
    }
}

#[cfg(feature = "minimal")]
pub fn digit<Rng: RngCore>(rng: &mut Rng) -> u32 {
    rng.random_range(0..=9)
}

#[cfg(feature = "minimal")]
pub fn number<Rng: RngCore>(rng: &mut Rng) -> u32 {
    rng.random_range(0..1_000_000_000)
}
