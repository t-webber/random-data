use rand::{Rng as _, RngCore};

use crate::{
    DataGenerator, DataType,
    primitives::{alphanumeric_capital_char, capital_char},
};

pub fn address<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{} {} {} {}",
        generator.rng().random_range(1..=999),
        DataType::Street.random(generator),
        DataType::City.random(generator),
        DataType::Country.random(generator),
    )
}

pub fn uk_address<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{} {}, {}, {}",
        generator.rng().random_range(1..=999),
        DataType::UkStreet.random(generator),
        DataType::UkCity.random(generator),
        DataType::UkPostCode.random(generator),
    )
}

pub fn french_address<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{} {}, {}, {}",
        generator.rng().random_range(1..=999),
        DataType::FrenchStreet.random(generator),
        DataType::FrenchCounty.random(generator),
        DataType::FrenchPostCode.random(generator),
    )
}

pub fn uk_post_code<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}{}{} {}{}{}",
        DataType::UkPostcodeArea.random(generator),
        generator.rng().random_range(1..=9),
        alphanumeric_capital_char(generator.rng()),
        generator.rng().random_range(1..=9),
        capital_char(generator.rng()),
        capital_char(generator.rng()),
    )
}

pub fn french_post_code<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{:02}{:03}",
        generator.rng().random_range(1..=95),
        generator.rng().random_range(1..500)
    )
}

pub fn latitude<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    generator.rng().random_range(-90.0..=90.0).to_string()
}

pub fn longitude<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    generator.rng().random_range(-180.0..=180.0).to_string()
}

pub fn latitude_longitude<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}, {}",
        DataType::Latitude.random(generator),
        DataType::Longitude.random(generator)
    )
}

pub fn street_number<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
    format!(
        "{}{}",
        generator.rng().random_range(0..1000),
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

pub fn french_street_name<Rng: RngCore>(generator: &mut DataGenerator<Rng>) -> String {
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
