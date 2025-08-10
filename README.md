# random-data

[![GitHub](https://img.shields.io/badge/github-t--webber-blue.svg?logo=github)](https://github.com/t-webber/random-data)
[![Crates.io](https://img.shields.io/badge/crates.io-random--data-darkgreen.svg?logo=rust)](https://crates.io/crates/random-data)
[![Docs.rs](https://img.shields.io/badge/docs.rs-random--data-brown.svg?logo=rust)](https://docs.rs/random-data)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-red.svg)](LICENSE-MIT)
![Coverage](https://img.shields.io/badge/coverage--100%25-purple.svg)

A Rust library for generating realistic random data of any kind.

## Looking for a CLI?

Checkout the [`random-json`](https://github.com/t-webber/random-json) crate to use this data as a CLI. It let's you choose the data you want with a dropdown dialogue in the terminal, or lets you generate big amounts of data in one go from a JSON schema!

## Installation

```bash
cargo add random-data
```

## Usage

```rust
use random_data::{DataType, DataGenerator};

fn main() {
    let mut generator = DataGenerator::new();

    // Generate a random address
    let address = DataType::Address.random(&mut generator);
    println!("Address: {address}");

    // Generate a random name
    let name = DataType::FirstName.random(&mut generator);
    println!("Name: {name}");

    // Generate a random email
    let email = DataType::Email.random(&mut generator);
    println!("Email: {email}");

    // Generate location data
    let country = DataType::Country.random(&mut generator);
    let coordinates = DataType::LatitudeLongitude.random(&mut generator);
    println!("Location: {country} at {coordinates}");

    // Generate technical data
    let ip = DataType::Ipv4.random(&mut generator);
    let mac = DataType::MacAddress.random(&mut generator);
    println!("Network: IP {ip} MAC {mac}");

    // List all the possible data types
    let list = DataType::list_str();
    println!("Possible data types: {list:?}");
}
```

## Checking Available Values

Some data types have predefined lists of values. You can access these lists:

```rust
use random_data::{DataGenerator, DataType};

let mut generator = DataGenerator::new();

// Check if a data type has a predefined list
if let Some(values) = DataType::Month.values() {
    println!("Available months: {:?}", values);
}

// Formula-based types return None
assert!(DataType::Address.values().is_none());

// Generate and verify
let month = DataType::Month.random(&mut generator);
let all_months = DataType::Month.values().unwrap();
assert!(all_months.contains(&month.as_ref()));
```

## Regional Data

The library includes region-specific generators:

```rust
use random_data::{DataGenerator, DataType};

let mut generator = DataGenerator::new();

// UK-specific data
let uk_address = DataType::UkAddress.random(&mut generator);
let uk_postcode = DataType::UkPostCode.random(&mut generator);
let uk_phone = DataType::UkPhoneNumber.random(&mut generator);

// French-specific data
let french_address = DataType::FrenchAddress.random(&mut generator);
let french_postcode = DataType::FrenchPostCode.random(&mut generator);
let french_phone = DataType::FrenchPhoneNumber.random(&mut generator);
```

## Available Data Types

- **Internet**: EmailDomain, HttpStatusCode, MimeType, OpenSourceApp, TopLevelDomain, UserAgent, Ipv4, Ipv6, MacAddress
- **Companies**: CarBrand, CompanyName, Job
- **Computer science**: FileExtension, ProgrammingLanguage, ProgrammingParadigm, EditorColourTheme, ItDomain, DirPath, FileName, FilePath, Semver, SemverStable, SemverUnstable
- **Famous people**: Painter, Writer, Composer, Mathematician, Physician, Biologist, ComputerScientist, Philosopher
- **Colours**: HexColour, HslaColour, HslColour, RgbaColour, RgbColour, ColourName
- **Geography**: Latitude, LatitudeLongitude, Longitude, FrenchRiver, Country, City, Continent, CountryCode, Street, River, UkRiver
- **Addresses**: Address, FrenchAddress, FrenchPostCode, UkAddress, UkPostCode, FrenchStreet, FrenchRoadType, FrenchCounty, UsStateAbbr, UsState, UsRoads, UsRoadTypes, UkCountyCode, UkPostcodeArea, UkCounty, UkCity, UkRoadType, UkStreet, StreetNumber
- **Science**: ChemicalElement, MathematicalFunction
- **Names**: FirstName, FrenchFirstName, FrenchLastName, LastName, NameTitle
- **Arts**: LiteraryGenre, ArchitecturalStyle, MusicalGenre, MusicalInstrument
- **Time**: DaysOfWeek, Month, DaysOfWeekAbbr, MonthsAbbr, TimeZone, TimeUnit, AmPm
- **Animals**: Animal, Mammal, Bird, Insect, MythologicalCreature, Fish, Amphibian, Reptile, Mollusc, AnimalType
- **Education**: SchoolSubject, AcademicDiscipline, DegreesTitle, University, HistoricalBattle, Sport
- **Primitives**: AlphanumericCapitalChar, AlphanumericChar, Boolean, CapitalChar, Digit, LowerChar
- **Personal data**: CreditCard, Email, FrenchEmail, FrenchLicencePlate, FrenchPhoneNumber, NhsNumber, Password, PhoneNumber, SecuriteSociale, UkLicencePlate, UkPhoneNumber
- **ISBNs**: RandomIsbn10, RandomIsbn13
- **Banks & finance**: Bic, Iban, Isin, CurrencyName, CurrencySymbol, CurrencyCode
- **Text**: Word, Sentence, Paragraph
- **Sky & space**: Constellation, Planet, Star, Galaxy, CloudType

> If you don't want all the data to be loaded in your binary, you can use the feature flags to disable those you don't want.
