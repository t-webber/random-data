use crate::{DataGenerator, primitives};

pub fn lower_char(generator: &mut DataGenerator) -> String {
    primitives::lower_char(generator.rng()).to_string()
}

pub fn capital_char(generator: &mut DataGenerator) -> String {
    primitives::capital_char(generator.rng()).to_string()
}

pub fn alphanumeric_char(generator: &mut DataGenerator) -> String {
    primitives::alphanumeric_char(generator.rng()).to_string()
}

pub fn alphanumeric_capital_char(generator: &mut DataGenerator) -> String {
    primitives::alphanumeric_capital_char(generator.rng()).to_string()
}

pub fn boolean(generator: &mut DataGenerator) -> String {
    primitives::boolean(generator.rng()).to_owned()
}

pub fn digit(generator: &mut DataGenerator) -> String {
    primitives::digit(generator.rng()).to_string()
}

pub fn number(generator: &mut DataGenerator) -> String {
    primitives::number(generator.rng()).to_string()
}
