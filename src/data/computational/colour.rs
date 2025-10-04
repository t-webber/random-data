use crate::DataGenerator;
use rand::Rng as _;
use rand::RngCore;

pub fn hex_colour<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
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

pub fn rgb_colour<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "rgb({}, {}, {})",
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
    )
}

pub fn rgba_colour<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "rgb({}, {}, {}, {})",
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random_range(0..256),
        generator.rng().random::<f32>()
    )
}

pub fn hsl_colour<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "hsl({}, {}%, {}%)",
        generator.rng().random_range(0..360),
        generator.rng().random_range(0..=100),
        generator.rng().random_range(0..=100),
    )
}

pub fn hsla_colour<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "hsl({}, {}%, {}%, {})",
        generator.rng().random_range(0..360),
        generator.rng().random_range(0..=100),
        generator.rng().random_range(0..=100),
        generator.rng().random::<f32>(),
    )
}
