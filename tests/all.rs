use std::collections::HashSet;

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
    let mut generator = DataGenerator::new();
    let list = dbg!(DataType::list());
    for data_type in list {
        for _ in 0..10000 {
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
