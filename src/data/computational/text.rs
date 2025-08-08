use rand::Rng as _;

use crate::{DataGenerator, DataType};

#[expect(clippy::unwrap_used, reason = "tests assure the first element exists")]
fn push_first_word(generator: &mut DataGenerator, output: &mut String) {
    let first_word = DataType::Word.random(generator);
    let mut chars = first_word.chars();
    for ch in chars.next().unwrap().to_uppercase() {
        output.push(ch);
    }
    for ch in chars {
        output.push(ch);
    }
    output.push(' ');
}

pub fn sentence(generator: &mut DataGenerator) -> String {
    let mut output = String::new();
    let len = generator.rng().random_range(10..20);
    push_first_word(generator, &mut output);

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
