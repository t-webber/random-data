use rand::Rng as _;

use crate::{DataGenerator, DataType};

pub fn phone_number(g: &mut DataGenerator) -> String {
    g.rng()
        .random_range(1_000_000u64..=999_999_999_999_999)
        .to_string()
}

pub fn uk_phone_number(g: &mut DataGenerator) -> String {
    format!(
        "44{}",
        g.rng().random_range(1_000_000_000u64..=9_999_999_999)
    )
}

pub fn french_phone_number(g: &mut DataGenerator) -> String {
    format!("33{}", g.rng().random_range(100_000_000u64..=999_999_999))
}

pub fn email(g: &mut DataGenerator) -> String {
    format!(
        "{}.{}@{}",
        DataType::FirstName.random(g),
        DataType::LastName.random(g),
        DataType::EmailDomain.random(g)
    )
}

pub fn french_email(g: &mut DataGenerator) -> String {
    format!(
        "{}.{}@{}",
        DataType::FrenchFirstName.random(g),
        DataType::FrenchLastName.random(g),
        DataType::EmailDomain.random(g)
    )
}
