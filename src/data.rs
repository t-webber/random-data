mod computational;
mod raw;
use crate::data::raw::*;
use crate::generator::DataGenerator;
use rand::Rng as _;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::fmt;

macro_rules! strings {
    ($($o_variant:ident: $o_func:ident;)*;  $($s_module:ident, $s_variant:ident, $s_const:ident)*) => {


        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum DataType {
            $($s_variant,)*
            $($o_variant,)*
        }

        impl DataType {
            const LIST: &[Self] = &[ $(Self::$s_variant,)* $(Self::$o_variant,)* ];

            pub fn values(&self) -> Option<&'static[&'static str]> {
                match self {
                    $( Self::$s_variant => Some(&$s_module::$s_const),)*
                        _ => None
                }
            }

            pub fn random(&self, g: &mut DataGenerator) -> String {
                match self {
                    $( DataType::$s_variant => ($s_module::$s_const).choose(g.rng()).unwrap().to_string(),)*
                    $( DataType::$o_variant => crate::data::computational::$o_func(g), )*
                }
            }
        }

        impl TryFrom<&str> for DataType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, ()> {
                match value {
                    $( stringify!($s_variant) => Ok(Self::$s_variant), )*
                    $( stringify!($o_variant) => Ok(Self::$o_variant), )*
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

strings!(
Address: address;
AlphanumericCapitalChar: alphanumeric_capital_char;
AlphanumericChar: alphanumeric_char;
Boolean: boolean;
CapitalChar: CapitalChar;
Digit: digit;
Email: email;
FrenchAddress: french_address;
FrenchEmail: french_email;
FrenchPhoneNumber: french_phone_number;
FrenchPostCode: french_post_code;
Latitude: latitude;
LatitudeLongitude: latitude_longitude;
Longitude: longitude;
LowerChar: lower_char;
PhoneNumber: phone_number;
RandomIsbn10: random_isbn10;
RandomIsbn13: random_isbn13;
UkAddress: uk_address;
UkPhoneNumber: uk_phone_number;
UkPostCode: uk_post_code;
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
internet, TopLevelDomain, TOP_LEVEL_DOMAINS
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
