use crate::{DataGenerator, primitives};
use rand::RngCore;

pub fn lower_char<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::lower_char(generator.rng()).to_string()
}

pub fn capital_char<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::capital_char(generator.rng()).to_string()
}

pub fn alphanumeric_char<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::alphanumeric_char(generator.rng()).to_string()
}

pub fn alphanumeric_capital_char<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::alphanumeric_capital_char(generator.rng()).to_string()
}

pub fn boolean<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::boolean(generator.rng()).to_owned()
}

pub fn digit<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::digit(generator.rng()).to_string()
}

pub fn number<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    primitives::number(generator.rng()).to_string()
}
