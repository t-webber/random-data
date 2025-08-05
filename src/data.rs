mod raw;

use rand::Rng as _;
use rand::rngs::ThreadRng;

use crate::generator::DataGenerator;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::fmt;

macro_rules! strings {
    ($($o_variant:ident: $o_func:expr;)*;  $($s_module:ident, $s_variant:ident, $s_const:ident)*) => {


        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum DataType {
            $($s_variant,)*
            $($o_variant,)*
        }

        impl DataType {
            const LIST: &[Self] = &[ $(Self::$s_variant,)* ];

            pub fn values(&self) -> Option<&'static[&'static str]> {
                match self {
                    $( Self::$s_variant => Some(&$s_module::$s_const),)*
                        _ => None
                }
            }

            pub fn random(&self, g: &mut DataGenerator) -> String {
                match self {
                    $( DataType::$s_variant => (crate::knowledge::$s_module::$s_const).choose(g.rng()).unwrap().to_string(),)*
                    $( DataType::$o_variant => $o_func(g), )*
                }
            }
        }

        impl TryFrom<&str> for DataType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, ()> {
                match value {
                    $( stringify!($s_variant) => Ok(Self::$s_variant), )*
                    _ => Err(())

                }
            }
        }

        impl DataGenerator {

        }
    };
}

impl DataType {
    pub const fn list() -> &'static [Self] {
        Self::LIST
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<&String> for DataType {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

fn random_isbn10(g: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(10);
    for _ in 0..9 {
        isbn.push(g.rng().gen_range(0usize..=9));
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

fn random_isbn13(g: &mut DataGenerator) -> String {
    let mut isbn = Vec::with_capacity(12);
    isbn.push(9);
    isbn.push(7);
    isbn.push(g.rng().gen_range(8..=9));
    for _ in 0..10 {
        isbn.push(g.rng().gen_range(0usize..=9));
    }
    let checksum = isbn
        .iter()
        .enumerate()
        .map(
            |(position, digit)| {
                if position % 2 == 0 { digit } else { digit * 3 }
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

strings!(
PhoneNumber: |g: &mut DataGenerator| g.rng().gen_range(1_000_000..=999_999_999_999_999).to_string();
UkPhoneNumber: |g: &mut DataGenerator| format!("44{}", g.rng().gen_range(1_000_000_000..=9_999_999_999));
FrenchPhoneNumber: |g: &mut DataGenerator| format!("33{}", g.rng().gen_range(100_000_000..=999_999_999));
Email: |g: &mut DataGenerator| format!("{}.{}@{}",
                                    DataType::FirstName.random(g),
                                    DataType::LastName.random(g),
                                    DataType::EmailDomain.random(g));
FrenchEmail: |g: &mut DataGenerator| format!("{}.{}@{}",
                                    DataType::FrenchFirstName.random(g),
                                    DataType::FrenchLastName.random(g),
                                    DataType::EmailDomain.random(g));
Address: |g: &mut DataGenerator| format!("{} {} {} {}",
                                    g.rng().gen_range(1..=999),
                                    DataType::Street.random(g),
                                    DataType::City.random(g),
                                    DataType::Country.random(g),
                                    );
UkAddress: |g: &mut DataGenerator| format!("{} {}, {}, {}",
                                    g.rng().gen_range(1..=999),
                                    DataType::UkStreet.random(g),
                                    DataType::UkCity.random(g),
                                    DataType::UkPostCode.random(g),
                                    );
FrenchAddress: |g: &mut DataGenerator| format!("{} {}, {}, {}",
                                    g.rng().gen_range(1..=999),
                                    DataType::FrenchStreet.random(g),
                                    DataType::FrenchCounty.random(g),
                                    DataType::FrenchPostCode.random(g),
                                    );
UkPostCode: |g: &mut DataGenerator| format!("{}{}{} {}{}{}",
                                    DataType::UkAreaCode.random(g),
                                    g.rng().gen_range(1..=9),
                                    DataType::AlphanumericCapitalChar.random(g),
                                    g.rng().gen_range(1..=9),
                                    DataType::CapitalChar.random(g),
                                    DataType::CapitalChar.random(g),
                                    );
FrenchPostCode: |g: &mut DataGenerator| format!("{:02}{:03}",
                                    g.rng().gen_range(1..=95),
                                    g.rng().gen_range(1..500));
LowerChar: |g: &mut DataGenerator| g.rng().gen_range('a'..='z') as char;
CapitalChar: |g: &mut DataGenerator| g.rng().gen_range('A'..='Z') as char;
AlphanumericChar: |g: &mut DataGenerator| *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                                    .choose(g.rng())
                                    .unwrap() as char;
AlphanumericCapitalChar: |g: &mut DataGenerator| *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                                    .choose(g.rng())
                                    .unwrap() as char;
Latitude: |g: &mut DataGenerator| g.rng().gen_range(-90.0..=90.0).to_string();
Longitude: |g: &mut DataGenerator| g.rng().gen_range(-180.0..=180.0).to_string();
LatitudeLongitude: |g: &mut DataGenerator| format!("{}, {}", DataType::Latitude.random(g), DataType::Longitude.random(g));
Isbn10: random_isbn10;
Isbn13: random_isbn13;
Boolean: |g: &mut DataGenerator| if g.rng().gen_bool(0.5) { "True".to_string() } else { "False".to_string() };
Digit: |g: &mut DataGenerator| g.rng().gen_range(0..=9).to_string();

;
art, ArchitecturalStyle, ARCHITECTURAL_STYLES
art, LiteraryGenre, LITERARY_GENRES
art, MusicalGenre, MUSICAL_GENRES
art, MusicalInstrument, MUSICAL_INSTRUMENTS
colour, ColourName, COLOUR_NAMES
currency, CurrencyCode, CURRENCY_CODES
currency, CurrencyName, CURRENCY_NAMES
currency, CurrencySymbol, CURRENCY_SYMBOLS
datetime, AmPm, AM_PM
datetime, DayOfWeekAbbr, DAYS_OF_WEEK_ABBR
datetime, DaysOfWeek, DAYS_OF_WEEK
datetime, MonthAbbr, MONTHS_ABBR
datetime, Month, MONTHS
datetime, TimeUnit, TIME_UNITS
datetime, TimeZone, TIME_ZONES
fauna, Amphibian, AMPHIBIANS
fauna, Animal, ANIMALS
fauna, AnimalType, ANIMAL_TYPES
fauna, Bird, BIRDS
fauna, Fish, FISHES
fauna, Insect, INSECTS
fauna, Mammal, MAMMALS
fauna, Mollusc, MOLLUSC
fauna, Reptile, REPTILES
france, FrenchCounty, FRENCH_COUNTIES
france, FrenchRoadType, FRENCH_ROAD_TYPES
france, FrenchStreet, FRENCH_STREETS
france, LongestFrenchRiver, LONGEST_FRENCH_RIVERS
history, HistoricalBattle, HISTORICAL_BATTLES
internet, EmailDomain, EMAIL_DOMAINS
internet, OpenSourceApp, OPEN_SOURCE_APPS
mythology, MythologicalCreature, MYTHOLOGICAL_CREATURES
name, FirstName, FIRST_NAMES
name, FrenchFirstName, FRENCH_FIRST_NAMES
name, FrenchLastName, FRENCH_LAST_NAMES
name, LastName, LAST_NAMES
people, Biologist, BIOLOGISTS
people, Composer, COMPOSERS
people, ComputerScientist, COMPUTER_SCIENTISTS
people, Mathematician, MATHEMATICIANS
people, Painter, PAINTERS
people, Philosopher, PHILOSOPHERS
people, Physician, PHYSICIANS
people, Writer, WRITERS
programming, ColourTheme, EDITOR_COLOUR_THEMES
programming, DomainName, IT_DOMAINS
programming, ProgrammingLanguage, PROGRAMMING_LANGUAGES
programming, ProgrammingParadigm, PROGRAMMING_PARADIGMS
science, ChemicalElement, CHEMICAL_ELEMENTS
science, MathFunction, MATHEMATICAL_FUNCTIONS
space, Constellation, CONSTELLATIONS
space, Galaxy, GALAXIES
space, Planet, PLANETS
space, Star, STARS
sport, Sport, SPORTS
uk, UkCity, UK_CITIES
uk, UkCountyCode, UK_COUNTY_CODES
uk, UkCounty, UK_COUNTIES
uk, UkAreaCode, UK_POSTCODE_AREAS
uk, UkRiver, LONGEST_UK_RIVERS
uk, UkRoadType, UK_ROAD_TYPES
uk, UkStreet, UK_STREETS
university, AcademicDiscipline, ACADEMIC_DISCIPLINES
university, DegreesTitle, DEGREES_TITLES
university, SchoolSubject, SCHOOL_SUBJECTS
university, University, UNIVERSITIES
us, UsRoadType, US_ROAD_TYPES
us, UsRoad, US_ROADS
us, UsStateAbbreviation, US_STATE_ABBR
us, UsState, AMERICAN_STATES
weather, Cloud, TYPES_OF_CLOUDS
work, CarBrand, CAR_BRANDS
work, CompanyName, COMPANY_NAMES
work, Job, JOBS
world, City, MOST_POPULATED_CITIES
world, Continent, CONTINENTS
world, CountryCode, COUNTRY_CODES
world, Country, COUNTRIES
world, River, LONGEST_RIVERS
world, Street, WORLD_STREETS

);
