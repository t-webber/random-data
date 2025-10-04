use rand::RngCore;
use rand::{Rng as _, seq::IndexedRandom as _};

use crate::{DataGenerator, DataType, primitives::capital_char};

pub fn isin<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}{:010}",
        DataType::CountryCode.random(generator),
        generator.rng().random_range(0u64..=9_999_999_999)
    )
}

pub fn bic<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}{}{}{}{country}{}{}{branch}",
        capital_char(generator.rng()),
        capital_char(generator.rng()),
        capital_char(generator.rng()),
        capital_char(generator.rng()),
        capital_char(generator.rng()),
        capital_char(generator.rng()),
        country = DataType::CountryCode.random(generator),
        branch = if generator.rng().random_bool(0.8) {
            "XXX".to_owned()
        } else {
            format!(
                "{}{}{}",
                capital_char(generator.rng()),
                capital_char(generator.rng()),
                capital_char(generator.rng()),
            )
        }
    )
}

#[allow(clippy::unwrap_used, reason = "non empty string")]
pub fn iban<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    let mut output = DataType::CountryCode.random(generator);
    for _ in 0..26u8 {
        output.push(
            *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .choose(generator.rng())
                .unwrap() as char,
        );
    }
    output
}
