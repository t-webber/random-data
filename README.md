# random-data

[![GitHub](https://img.shields.io/badge/github-t--webber-blue.svg?logo=github)](https://github.com/t-webber/random-data)
[![Crates.io](https://img.shields.io/badge/crates.io-random--data-darkgreen.svg?logo=rust)](https://crates.io/crates/random-data)
[![Docs.rs](https://img.shields.io/badge/docs.rs-random--data-brown.svg?logo=rust)](https://docs.rs/random-data)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-red.svg)](LICENSE-MIT)

A Rust library for generating realistic random data of any kind.

## Looking for a CLI?

Checkout the [`random-json`](https://github.com/t-webber/random-json) crate to use this data as a CLI. It let's you choose the data you want with a dropdown dialogue in the terminal, or lets you generate big amounts of data in one go from a JSON schema!

## Features

- **Extensive Data Types**: Over 100 different types of random data including:
  - **Personal Information**: Names, emails, phone numbers, addresses
  - **Geographic Data**: Countries, cities, coordinates, postcodes
  - **Technical Data**: IP addresses, MAC addresses, file paths, semvers, programming languages
  - **Financial Data**: IBAN, BIC, credit cards, currencies
  - **Colors**: RGB, HSL, Hex, colors with alpha channels
  - **Academic**: Universities, subjects, degrees
  - **And much more**: Animals, planets, famous people, chemical elements, etc.

- **Formula-based and List-based Generation**: Some data is generated using formulas (like addresses, coordinates), while others are randomly selected from lists of read-data values (like animal names, countries, companies) for the data to be as real as possible.

- **Locale Support** (limited): Some generators exist for different regions (UK, France, US)

- **Simple API**: Easy-to-use interface with a single `DataType` enum, and a single `random` method.

- **No External Dependencies**: Only depends on the `rand` crate for randomisation

## Quick Start

```bash
cargo add random-data
```

## Usage

```rust
use random_data::{DataGenerator, DataType};

fn main() {
    let mut generator = DataGenerator::new();

    // Generate a random address
    let address = DataType::Address.random(&mut generator);
    println!("Address: {}", address);

    // Generate a random name
    let name = DataType::FirstName.random(&mut generator);
    println!("Name: {}", name);

    // Generate a random email
    let email = DataType::Email.random(&mut generator);
    println!("Email: {}", email);

    // Generate location data
    let country = DataType::Country.random(&mut generator);
    let coordinates = DataType::LatitudeLongitude.random(&mut generator);
    println!("Location: {} at {}", country, coordinates);

    // Generate technical data
    let ip = DataType::Ipv4.random(&mut generator);
    let mac = DataType::MacAddress.random(&mut generator);
    println!("Network: IP {} MAC {}", ip, mac);

    // List all the possible data types
    let list = DataType::list_str();
    println!("Possible data types: {}", list);
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

**Internet**: EmailDomain, HttpStatusCode, MimeType, OpenSourceApp, TopLevelDomain, UserAgent,
**France**: FrenchStreet, FrenchRoadType, FrenchCounty, LongestFrenchRiver,
**Companies**: CarBrand, CompanyName, Job,
**Computer science**: FileExtension, ProgrammingLanguage, ProgrammingParadigm, EditorColourTheme, ItDomain,
Painter, Writer, Composer, Mathematician, Physician, Biologist, ComputerScientist, Philosopher,
HexColour, HslaColour, HslColour, RgbaColour, RgbColour, ColourName,
Address, FrenchAddress, FrenchPostCode, Latitude, LatitudeLongitude, Longitude, UkAddress, UkPostCode,
UsStateAbbr, AmericanState, UsRoads, UsRoadTypes,
ChemicalElement, MathematicalFunction,
FirstName, FrenchFirstName, FrenchLastName, LastName, NameTitle,
MythologicalCreature,
LiteraryGenre, ArchitecturalStyle, MusicalGenre, MusicalInstrument,
DaysOfWeek, Month, DaysOfWeekAbbr, MonthsAbbr, TimeZone, TimeUnit, AmPm,
SchoolSubject, AcademicDiscipline, DegreesTitle, University,
HistoricalBattle,
Animal, Mammal, Bird, Insect, Fishe, Amphibian, Reptile, Mollusc, AnimalType,
AlphanumericCapitalChar, AlphanumericChar, Boolean, CapitalChar, Digit, LowerChar,
CreditCard, Email, FrenchEmail, FrenchLicencePlate, FrenchPhoneNumber, NhsNumber, Password, PhoneNumber, SecuriteSociale, Sentence, UkLicencePlate, UkPhoneNumber,
RandomIsbn10, RandomIsbn13,
Bic, Iban, Isin,
DirPath, FileName, FilePath, Ipv4, Ipv6, MacAddress, Semver, SemverStable, SemverUnstable,
Country, City, Continent, CountryCode, Street, LongestRiver,
TypesOfCloud,
Word,
UkCountyCode, UkPostcodeArea, UkCounty, UkCity, UkRoadType, LongestUkRiver, UkStreet,
Sport,
Constellation, Planet, Star, Galaxy,
CurrencyName, CurrencySymbol, CurrencyCode,
