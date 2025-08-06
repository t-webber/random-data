use crate::{DataGenerator, DataType};
use rand::{Rng as _, seq::IndexedRandom as _};

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
        DataType::UkAreaCode.random(g),
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
pub fn lower_char(g: &mut DataGenerator) -> String {
    g.rng().random_range('a'..='z').to_string()
}

pub fn CapitalChar(g: &mut DataGenerator) -> String {
    g.rng().random_range('A'..='Z').to_string()
}

pub fn alphanumeric_char(g: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
        .choose(g.rng())
        .unwrap() as char)
        .to_string()
}
pub fn alphanumeric_capital_char(g: &mut DataGenerator) -> String {
    (*b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .choose(g.rng())
        .unwrap() as char)
        .to_string()
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

pub fn boolean(g: &mut DataGenerator) -> String {
    if g.rng().gen_bool(0.5) {
        "True".to_string()
    } else {
        "False".to_string()
    }
}

pub fn digit(g: &mut DataGenerator) -> String {
    g.rng().random_range(0..=9).to_string()
}

pub fn random_isbn10(g: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(10);
    for _ in 0..9 {
        isbn.push(g.rng().random_range(0usize..=9));
    }
    let checksum = isbn
        .iter()
        .enumerate()
        .map(|(position, digit)| (digit * (position + 1)))
        .sum::<usize>()
        % 11;
    format!(
        "{}-{}{}{}-{}{}{}{}{}-{}",
        isbn[0],
        isbn[1],
        isbn[2],
        isbn[3],
        isbn[4],
        isbn[5],
        isbn[6],
        isbn[7],
        isbn[8],
        if checksum == 10 {
            'X'
        } else {
            char::from_digit(checksum as u32, 10).unwrap()
        }
    )
}

pub fn random_isbn13(g: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(12);
    isbn.push(9);
    isbn.push(7);
    isbn.push(g.rng().random_range(8..=9));
    for _ in 0..10 {
        isbn.push(g.rng().random_range(0usize..=9));
    }
    let checksum = isbn
        .iter()
        .enumerate()
        .map(
            |(position, digit)| {
                if position % 2 == 0 { *digit } else { digit * 3 }
            },
        )
        .sum::<usize>()
        % 10;
    let check_digit = (10 - checksum) % 10;
    format!(
        "{}{}{}-{}-{}{}-{}{}{}{}{}{}-{}",
        isbn[0],
        isbn[1],
        isbn[2],
        isbn[3],
        isbn[4],
        isbn[5],
        isbn[6],
        isbn[7],
        isbn[8],
        isbn[9],
        isbn[10],
        isbn[11],
        check_digit
    )
}
