use rand::Rng;

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

pub fn nhs_number(g: &mut DataGenerator) -> String {
    let digits = (0..8)
        .map(|_| g.rng().gen_range(0..=9))
        .collect::<Vec<i32>>();
    let sum: i32 = digits
        .iter()
        .enumerate()
        .map(|(idx, digit)| (10 - idx as i32) * digit)
        .sum();
    let sum = sum % 11;
    let nine = if sum == 3 { 3 - sum } else { 2 - sum } % 11;
    let ten = (11 - sum - nine) % 11;
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

pub fn securite_sociale(g: &mut DataGenerator) -> String {
    let rng = g.rng();
    let nir = format!(
        "{}{:02}{:02}{:02}{:03}{:03}",
        rng.gen_range(1..=2),
        rng.gen_range(0..=99),
        rng.gen_range(1..=12),
        rng.gen_range(1..=95),
        rng.gen_range(1..900),
        rng.gen_range(1..900),
    );
    let value = nir.parse::<u64>().unwrap();
    let key = 97 - (value % 97);
    format!("{nir}{key}")
}
