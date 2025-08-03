use std::collections::HashSet;

use fake_data::{DataGenerator, DataType};

#[test]
fn string() {
    let list = DataType::list();
    for data_type in list {
        assert!(DataType::try_from(&data_type.to_string()).unwrap() == *data_type);
    }
}

#[test]
fn non_empty() {
    let mut generator = DataGenerator::new();
    let list = DataType::list();
    for data_type in list {
        for _ in 0..1000 {
            assert!(!generator.random(data_type).is_empty());
        }
    }
}

#[test]
fn uniquessness() {
    let list = DataType::list();
    for data_type in list {
        let mut set = HashSet::new();
        for value in data_type.values() {
            if !set.insert(value) {
                panic!("duplicate value: {value}");
            }
        }
    }
}
