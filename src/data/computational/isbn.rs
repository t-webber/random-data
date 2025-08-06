use rand::Rng as _;

use crate::DataGenerator;

#[expect(clippy::indexing_slicing, reason = "index in bounds")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::unwrap_used,
    reason = "small enough"
)]
pub fn random_isbn10(generator: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(10);
    for _ in 0..9u32 {
        isbn.push(generator.rng().random_range(0usize..=9));
    }
    let checksum = isbn
        .iter()
        .enumerate()
        .map(|(position, digit)| digit * (position + 1))
        .sum::<usize>()
        .rem_euclid(11);
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

#[expect(clippy::indexing_slicing, reason = "index in bounds")]
pub fn random_isbn13(generator: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(12);
    isbn.push(9);
    isbn.push(7);
    isbn.push(generator.rng().random_range(8..=9));
    for _ in 0u32..10 {
        isbn.push(generator.rng().random_range(0usize..=9));
    }
    let checksum = isbn
        .iter()
        .enumerate()
        .map(
            |(position, digit)| {
                if position & 1 == 0 { *digit } else { digit * 3 }
            },
        )
        .sum::<usize>()
        .rem_euclid(10);
    let check_digit = (10 - checksum).rem_euclid(10);
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
