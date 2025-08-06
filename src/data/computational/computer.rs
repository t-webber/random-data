use rand::Rng;

use crate::{DataGenerator, DataType};

pub fn semver(generator: &mut DataGenerator) -> String {
    if generator.rng().random_bool(0.8) {
        DataType::SemverStable.random(generator)
    } else {
        DataType::SemverUnstable.random(generator)
    }
}

pub fn semver_stable(generator: &mut DataGenerator) -> String {
    format!(
        "{}.{}.{}",
        generator.rng().random_range(0..20),
        generator.rng().random_range(1..20),
        generator.rng().random_range(0..20)
    )
}

#[expect(clippy::unreachable, reason = "it is unreachable")]
pub fn semver_unstable(generator: &mut DataGenerator) -> String {
    format!(
        "{}-{}{}",
        DataType::SemverStable.random(generator),
        match generator.rng().random_range(0..5) {
            0 => "alpha",
            1 => "beta",
            2 => "rc",
            3 => "nightly",
            4 => "dev",
            _ => unreachable!(),
        },
        if generator.rng().random_bool(0.9) {
            format!(".{}", generator.rng().random_range(0..20))
        } else {
            String::new()
        }
    )
}

pub fn mac_address(generator: &mut DataGenerator) -> String {
    format!(
        "{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}",
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
        generator.rng().random_range(0..16),
    )
}

pub fn ipv4(generator: &mut DataGenerator) -> String {
    format!(
        "{}.{}.{}.{}",
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
    )
}

pub fn ipv6(generator: &mut DataGenerator) -> String {
    format!(
        "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
        generator.rng().random_range(0..=0xffff),
    )
}

pub fn dir_path(generator: &mut DataGenerator) -> String {
    let len = generator.rng().random_range(0u32..=5);
    let mut output = String::from("/");
    for _ in 0..len {
        output.push_str(&DataType::Word.random(generator));
        output.push('/');
    }
    output
}
