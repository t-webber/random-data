use rand::Rng;

use crate::{DataGenerator, DataType};

pub fn semver(g: &mut DataGenerator) -> String {
    if g.rng().random_bool(0.8) {
        DataType::SemverStable.random(g)
    } else {
        DataType::SemverUnstable.random(g)
    }
}

pub fn semver_stable(g: &mut DataGenerator) -> String {
    format!(
        "{}.{}.{}",
        g.rng().random_range(0..20),
        g.rng().random_range(1..20),
        g.rng().random_range(0..20)
    )
}

pub fn semver_unstable(g: &mut DataGenerator) -> String {
    format!(
        "{}-{}{}",
        DataType::SemverStable.random(g),
        match g.rng().random_range(0..5) {
            0 => "alpha",
            1 => "beta",
            2 => "rc",
            3 => "nightly",
            4 => "dev",
            _ => unreachable!(),
        },
        if g.rng().random_bool(0.9) {
            format!(".{}", g.rng().random_range(0..20))
        } else {
            String::new()
        }
    )
}

pub fn mac_address(g: &mut DataGenerator) -> String {
    let r = |g: &mut DataGenerator| g.rng().random_range(0..16);
    format!(
        "{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}",
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
        g.rng().random_range(0..16),
    )
}

pub fn ipv4(g: &mut DataGenerator) -> String {
    format!(
        "{}.{}.{}.{}",
        g.rng().random_range(0..=255),
        g.rng().random_range(0..=255),
        g.rng().random_range(0..=255),
        g.rng().random_range(0..=255),
    )
}

pub fn ipv6(g: &mut DataGenerator) -> String {
    format!(
        "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
        g.rng().random_range(0..=0xffff),
    )
}
