use rand::{
    Rng as _, RngCore,
    distr::{Distribution as _, weighted::WeightedIndex},
    seq::IndexedRandom as _,
};

use crate::{DataGenerator, DataType};

pub fn semver<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    if generator.rng().random_bool(0.8) {
        DataType::SemverStable.random(generator)
    } else {
        DataType::SemverUnstable.random(generator)
    }
}

pub fn semver_stable<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}.{}.{}",
        generator.rng().random_range(0..20),
        generator.rng().random_range(1..20),
        generator.rng().random_range(0..20)
    )
}

#[allow(clippy::unwrap_used, reason = "array not empty")]
pub fn semver_unstable<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}-{}{}",
        DataType::SemverStable.random(generator),
        ["alpha", "beta", "rc", "nightly", "dev"]
            .choose(generator.rng())
            .unwrap(),
        if generator.rng().random_bool(0.9) {
            format!(".{}", generator.rng().random_range(0..20))
        } else {
            String::new()
        }
    )
}

pub fn mac_address<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
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

pub fn ipv4<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}.{}.{}.{}",
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
        generator.rng().random_range(0..=255),
    )
}

pub fn ipv6<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
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

#[allow(clippy::unwrap_used, reason = "non empty, sum non zero, no overflow")]
pub fn dir_path<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    let weights = [1u32, 10, 100, 1000, 1000, 1000];
    let dist = WeightedIndex::new(weights).unwrap();
    let len = dist.sample(generator.rng());

    let mut output = String::from("/");
    for _ in 0..len {
        output.push_str(&DataType::Word.random(generator));
        output.push('/');
    }
    output
}

pub fn file_name<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}.{}",
        DataType::Word.random(generator),
        DataType::FileExtension.random(generator)
    )
}

pub fn file_path<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}{}",
        DataType::DirPath.random(generator),
        DataType::FileName.random(generator)
    )
}
