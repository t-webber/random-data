use rand::Rng as _;

use crate::{DataGenerator, DataType};

pub fn address(g: &mut DataGenerator) -> String {
    format!(
        "{} {} {} {}",
        g.rng().random_range(1..=999),
        DataType::Street.random(g),
        DataType::City.random(g),
        DataType::Country.random(g),
    )
}

pub fn uk_address(g: &mut DataGenerator) -> String {
    format!(
        "{} {}, {}, {}",
        g.rng().random_range(1..=999),
        DataType::UkStreet.random(g),
        DataType::UkCity.random(g),
        DataType::UkPostCode.random(g),
    )
}

pub fn french_address(g: &mut DataGenerator) -> String {
    format!(
        "{} {}, {}, {}",
        g.rng().random_range(1..=999),
        DataType::FrenchStreet.random(g),
        DataType::FrenchCounty.random(g),
        DataType::FrenchPostCode.random(g),
    )
}

pub fn uk_post_code(g: &mut DataGenerator) -> String {
    format!(
        "{}{}{} {}{}{}",
        DataType::UkPostcodeArea.random(g),
        g.rng().random_range(1..=9),
        DataType::AlphanumericCapitalChar.random(g),
        g.rng().random_range(1..=9),
        DataType::CapitalChar.random(g),
        DataType::CapitalChar.random(g),
    )
}

pub fn french_post_code(g: &mut DataGenerator) -> String {
    format!(
        "{:02}{:03}",
        g.rng().random_range(1..=95),
        g.rng().random_range(1..500)
    )
}

pub fn latitude(g: &mut DataGenerator) -> String {
    g.rng().random_range(-90.0..=90.0).to_string()
}

pub fn longitude(g: &mut DataGenerator) -> String {
    g.rng().random_range(-180.0..=180.0).to_string()
}

pub fn latitude_longitude(g: &mut DataGenerator) -> String {
    format!(
        "{}, {}",
        DataType::Latitude.random(g),
        DataType::Longitude.random(g)
    )
}
