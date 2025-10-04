use rand::RngCore;
use rand::seq::IndexedRandom as _;

use crate::{DataGenerator, DataType};

#[allow(clippy::unwrap_used, reason = "array not empty")]
pub fn famous_person<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
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
