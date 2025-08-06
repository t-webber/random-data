use core::iter::repeat_with;

use rand::Rng as _;

use crate::{DataGenerator, DataType};

pub fn phone_number(generator: &mut DataGenerator) -> String {
    generator
        .rng()
        .random_range(1_000_000u64..=999_999_999_999_999)
        .to_string()
}

pub fn uk_phone_number(generator: &mut DataGenerator) -> String {
    format!(
        "44{}",
        generator
            .rng()
            .random_range(1_000_000_000u64..=9_999_999_999)
    )
}

pub fn french_phone_number(generator: &mut DataGenerator) -> String {
    format!(
        "33{}",
        generator.rng().random_range(100_000_000u64..=999_999_999)
    )
}

pub fn email(generator: &mut DataGenerator) -> String {
    format!(
        "{}.{}@{}",
        DataType::FirstName.random(generator),
        DataType::LastName.random(generator),
        DataType::EmailDomain.random(generator)
    )
}

pub fn french_email(generator: &mut DataGenerator) -> String {
    format!(
        "{}.{}@{}",
        DataType::FrenchFirstName.random(generator),
        DataType::FrenchLastName.random(generator),
        DataType::EmailDomain.random(generator)
    )
}

#[expect(clippy::indexing_slicing, reason = "index in bounds")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "number is small"
)]
pub fn nhs_number(generator: &mut DataGenerator) -> String {
    let digits: Vec<i32> = repeat_with(|| generator.rng().random_range(0i32..=9i32))
        .take(8)
        .collect();
    let sum = digits
        .iter()
        .enumerate()
        .map(|(idx, digit)| (10i32 - idx as i32) * digit)
        .sum::<i32>()
        .rem_euclid(11);
    let nine = if sum == 3 { 3 - sum } else { 2 - sum }.rem_euclid(11);
    let ten = (11 - sum - nine).rem_euclid(11);
    format!(
        "{}{}{} {}{}{} {}{}{}{}",
        digits[0],
        digits[1],
        digits[2],
        digits[3],
        digits[4],
        digits[5],
        digits[6],
        digits[7],
        nine,
        ten
    )
}

#[expect(clippy::unwrap_used, reason = "it's a valid number")]
pub fn securite_sociale(generator: &mut DataGenerator) -> String {
    let rng = generator.rng();
    let nir = format!(
        "{}{:02}{:02}{:02}{:03}{:03}",
        rng.random_range(1u32..=2),
        rng.random_range(0u32..=99),
        rng.random_range(1u32..=12),
        rng.random_range(1u32..=95),
        rng.random_range(1u32..900),
        rng.random_range(1u32..900),
    );
    let value = nir.parse::<u64>().unwrap();
    let key = 97 - value.rem_euclid(97);
    format!("{nir}{key}")
}
