use rand::Rng as _;

use crate::{DataGenerator, DataType};

pub fn address(generator: &mut DataGenerator) -> String {
    format!(
        "{} {} {} {}",
        generator.rng().random_range(1..=999),
        DataType::Street.random(generator),
        DataType::City.random(generator),
        DataType::Country.random(generator),
    )
}

pub fn uk_address(generator: &mut DataGenerator) -> String {
    format!(
        "{} {}, {}, {}",
        generator.rng().random_range(1..=999),
        DataType::UkStreet.random(generator),
        DataType::UkCity.random(generator),
        DataType::UkPostCode.random(generator),
    )
}

pub fn french_address(generator: &mut DataGenerator) -> String {
    format!(
        "{} {}, {}, {}",
        generator.rng().random_range(1..=999),
        DataType::FrenchStreet.random(generator),
        DataType::FrenchCounty.random(generator),
        DataType::FrenchPostCode.random(generator),
    )
}

pub fn uk_post_code(generator: &mut DataGenerator) -> String {
    format!(
        "{}{}{} {}{}{}",
        DataType::UkPostcodeArea.random(generator),
        generator.rng().random_range(1..=9),
        DataType::AlphanumericCapitalChar.random(generator),
        generator.rng().random_range(1..=9),
        DataType::CapitalChar.random(generator),
        DataType::CapitalChar.random(generator),
    )
}

pub fn french_post_code(generator: &mut DataGenerator) -> String {
    format!(
        "{:02}{:03}",
        generator.rng().random_range(1..=95),
        generator.rng().random_range(1..500)
    )
}

pub fn latitude(generator: &mut DataGenerator) -> String {
    generator.rng().random_range(-90.0..=90.0).to_string()
}

pub fn longitude(generator: &mut DataGenerator) -> String {
    generator.rng().random_range(-180.0..=180.0).to_string()
}

pub fn latitude_longitude(generator: &mut DataGenerator) -> String {
    format!(
        "{}, {}",
        DataType::Latitude.random(generator),
        DataType::Longitude.random(generator)
    )
}

pub fn street_number(generator: &mut DataGenerator) -> String {
    format!(
        "{}{}",
        generator.rng().random_range(0..500),
        if generator.rng().random_bool(0.2) {
            if generator.rng().random_bool(0.4) {
                " ter"
            } else {
                " bis"
            }
        } else {
            ""
        }
    )
}

pub fn french_street_name(generator: &mut DataGenerator) -> String {
    if generator.rng().random_bool(0.3) {
        DataType::FamousFrenchStreet.random(generator)
    } else {
        format!(
            "{} {}",
            DataType::FrenchRoadType.random(generator),
            DataType::FamousPerson.random(generator)
        )
    }
}
