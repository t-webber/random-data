use rand::{Rng as _, seq::IndexedRandom as _};

use crate::{DataGenerator, DataType};

pub fn lower_char(g: &mut DataGenerator) -> String {
    g.rng().random_range('a'..='z').to_string()
}

pub fn capital_char(g: &mut DataGenerator) -> String {
    g.rng().random_range('A'..='Z').to_string()
}

pub fn alphanumeric_char(g: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .choose(g.rng())
        .unwrap() as char)
        .to_string()
}

pub fn alphanumeric_capital_char(g: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .choose(g.rng())
        .unwrap() as char)
        .to_string()
}

pub fn boolean(g: &mut DataGenerator) -> String {
    if g.rng().gen_bool(0.5) {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

pub fn digit(g: &mut DataGenerator) -> String {
    g.rng().random_range(0..=9).to_string()
}
