use rand::seq::IndexedRandom as _;

use crate::{DataGenerator, DataType};

#[expect(clippy::unwrap_used, reason = "array not empty")]
pub fn famous_person(generator: &mut DataGenerator) -> String {
    [
        DataType::Painter,
        DataType::Writer,
        DataType::Composer,
        DataType::Mathematician,
        DataType::Physician,
        DataType::Biologist,
        DataType::ComputerScientist,
        DataType::Philosopher,
    ]
    .choose(generator.rng())
    .unwrap()
    .random(generator)
}
