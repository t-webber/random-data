use rand::Rng as _;

use crate::{DataGenerator, DataType};

pub fn sentence(generator: &mut DataGenerator) -> String {
    let mut output = String::new();
    let len = generator.rng().random_range(10..20);
    for _ in 0u32..len {
        output.push_str(&DataType::Word.random(generator));
        output.push(' ');
    }
    output.push_str(&DataType::Word.random(generator));
    output.push('.');
    output
}

pub fn paragraph(generator: &mut DataGenerator) -> String {
    let mut output = String::new();
    let len = generator.rng().random_range(5..10);
    for _ in 0u32..len {
        output.push_str(&DataType::Sentence.random(generator));
        output.push(' ');
    }
    output.push_str(&DataType::Sentence.random(generator));
    output
}
