use rand::{Rng as _, seq::IndexedRandom as _};

use crate::DataGenerator;

pub fn lower_char(generator: &mut DataGenerator) -> String {
    generator.rng().random_range('a'..='z').to_string()
}

pub fn capital_char(generator: &mut DataGenerator) -> String {
    generator.rng().random_range('A'..='Z').to_string()
}

#[expect(clippy::unwrap_used, reason = "non empty string")]
pub fn alphanumeric_char(generator: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .choose(generator.rng())
        .unwrap() as char)
        .to_string()
}

#[expect(clippy::unwrap_used, reason = "non empty string")]
pub fn alphanumeric_capital_char(generator: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .choose(generator.rng())
        .unwrap() as char)
        .to_string()
}

pub fn boolean(generator: &mut DataGenerator) -> String {
    if generator.rng().random_bool(0.5) {
        "True"
    } else {
        "False"
    }
    .to_owned()
}

pub fn digit(generator: &mut DataGenerator) -> String {
    generator.rng().random_range(0..=9).to_string()
}

pub fn number(generator: &mut DataGenerator) -> String {
    generator.rng().random_range(0..1_000_000_000).to_string()
}
