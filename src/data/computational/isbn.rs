use rand::Rng as _;

use crate::DataGenerator;

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
