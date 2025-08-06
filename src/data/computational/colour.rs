use rand::Rng as _;

use crate::DataGenerator;

pub fn hex_colour(generator: &mut DataGenerator) -> String {
    format!(
        "#{:X}{:X}{:X}{:X}{:X}{:X}",
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
    )
}

pub fn rgb_colour(generator: &mut DataGenerator) -> String {
    format!(
        "rgb({}, {}, {})",
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
    )
}

pub fn rgba_colour(generator: &mut DataGenerator) -> String {
    format!(
        "rgb({}, {}, {}, {})",
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random::<f32>()
    )
}

pub fn hsl_colour(generator: &mut DataGenerator) -> String {
    format!(
        "hsl({}, {}%, {}%)",
        generator.rng().random_range(0..360),
        generator.rng().random_range(0..=100),
        generator.rng().random_range(0..=100),
    )
}

pub fn hsla_colour(generator: &mut DataGenerator) -> String {
    format!(
        "hsl({}, {}%, {}%, {})",
        generator.rng().random_range(0..360),
        generator.rng().random_range(0..=100),
        generator.rng().random_range(0..=100),
        generator.rng().random::<f32>(),
    )
}
