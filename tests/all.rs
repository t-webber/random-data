use std::collections::HashSet;

use random_data::rand::{Rng, SeedableRng as _};
use random_data::rand_chacha::ChaCha20Rng;
use random_data::{DataGenerator, DataType};

#[test]
fn string() {
    let list = DataType::list();
    for data_type in list {
        assert!(DataType::try_from(&data_type.to_string()).unwrap() == *data_type);
    }

    assert!(DataType::try_from("invalid_string").is_err());
    assert!(DataType::try_from("").is_err());
}

#[test]
fn list() {
    for (i, repr) in DataType::list_str().iter().enumerate() {
        assert_eq!(*repr, DataType::list()[i].to_string().as_str());
    }
}

#[test]
fn non_empty() {
    let mut generator = DataGenerator::default();
    let list = dbg!(DataType::list());
    for data_type in list {
        for _ in 0..1000 {
            assert!(!data_type.random(&mut generator).is_empty());
        }
    }
}

#[test]
fn uniquessness() {
    let list = DataType::list();
    for data_type in list {
        let mut set = HashSet::new();
        if let Some(values) = data_type.values() {
            for value in values {
                if !set.insert(value) {
                    panic!("duplicate value: {value}");
                }
            }
        }
    }
}

fn len(data_type: &DataType) -> Option<usize> {
    if let Some(values) = data_type.values() {
        Some(values.len())
    } else {
        #[cfg(feature = "minimal")]
        {
            Some(match data_type {
                DataType::AlphanumericCapitalChar => 36,
                DataType::AlphanumericChar => 62,
                DataType::Boolean => 2,
                DataType::CapitalChar => 26,
                DataType::Digit => 10,
                DataType::LowerChar => 26,
                _ => return None,
            })
        }
        #[cfg(not(feature = "minimal"))]
        {
            None
        }
    }
}

#[test]
fn random() {
    let mut generator = DataGenerator::default();
    let list = DataType::list();
    for data_type in list {
        let mut set = HashSet::new();
        for _ in 0..100 {
            let data = data_type.random(&mut generator);
            set.insert(data.clone());
        }
        let expected = len(data_type).map_or_else(|| 70, |len| (len / 4).min(70));
        assert!(
            set.len() > expected,
            "{data_type} expected {expected}, found {}",
            set.len()
        );
    }
}

#[test]
fn different() {
    let mut generator0 = DataGenerator::default();
    let mut generator1 = DataGenerator::default();

    if DataType::list().len() <= 10 {
        return;
    }

    let values0 = DataType::list()
        .iter()
        .map(|data_type| data_type.random(&mut generator0))
        .take(10)
        .collect::<Vec<_>>();

    let values1 = DataType::list()
        .iter()
        .map(|data_type| data_type.random(&mut generator1))
        .take(10)
        .collect::<Vec<_>>();

    assert_ne!(values0, values1);
}

#[test]
fn seed() {
    let mut generator0 = DataGenerator::new_with_seed(287201);
    let mut generator1 = DataGenerator::new_with_seed(287201);

    let values0 = DataType::list()
        .iter()
        .map(|data_type| data_type.random(&mut generator0))
        .take(100)
        .collect::<Vec<_>>();

    let values1 = DataType::list()
        .iter()
        .map(|data_type| data_type.random(&mut generator1))
        .take(100)
        .collect::<Vec<_>>();

    assert_eq!(values0, values1);
}

#[test]
fn external_generator() {
    let mut raw_rng = ChaCha20Rng::seed_from_u64(0xdeadbeef);
    let mut generator = DataGenerator::from(raw_rng.clone());
    let data_rng = generator.rng();

    let raw_values = (0..100)
        .map(|_| raw_rng.random_bool(0.5))
        .collect::<Vec<_>>();

    let data_values = (0..100)
        .map(|_| data_rng.random_bool(0.5))
        .collect::<Vec<_>>();

    assert_eq!(raw_values, data_values)
}
