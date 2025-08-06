use rand::{Rng as _, seq::IndexedRandom as _};

use crate::{DataGenerator, DataType};

pub fn isin(g: &mut DataGenerator) -> String {
    format!(
        "{}{:010}",
        DataType::CountryCode.random(g),
        g.rng().random_range(0u64..=9_999_999_999)
    )
}

pub fn bic(g: &mut DataGenerator) -> String {
    format!(
        "{}{}{}{}{country}{}{}{branch}",
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
        country = DataType::CountryCode.random(g),
        branch = if g.rng().random_bool(0.8) {
            "XXX".to_owned()
        } else {
            format!(
                "{}{}{}",
                DataType::CapitalChar.random(g),
                DataType::CapitalChar.random(g),
                DataType::CapitalChar.random(g)
            )
        }
    )
}

pub fn iban(g: &mut DataGenerator) -> String {
    let mut output = DataType::CountryCode.random(g);
    for i in 0..26 {
        output.push(
            *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .choose(g.rng())
                .unwrap() as char,
        );
    }
    output
}
