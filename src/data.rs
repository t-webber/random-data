mod computational;
mod raw;
use crate::data::computational::*;
use crate::data::raw::*;
use crate::generator::DataGenerator;
use rand::Rng as _;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::fmt;

macro_rules! strings {
    ($($o_module:ident, $o_variant:ident, $o_func:ident)*;  $($s_module:ident, $s_variant:ident, $s_const:ident)*) => {


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
                    $( DataType::$o_variant => $o_module::$o_func(g), )*
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
// cd src/data/computational && ls | while read f; do cat $f | grep '^pub fn' | tr '(' ' ' | awk '{print $3}' | sort | while read l; do echo "$f, $(caseify -p "$l"), $l"; done; done
address, Address, address
address, FrenchAddress, french_address
address, FrenchPostCode, french_post_code
address, Latitude, latitude
address, LatitudeLongitude, latitude_longitude
address, Longitude, longitude
address, UkAddress, uk_address
address, UkPostCode, uk_post_code
isbn, RandomIsbn10, random_isbn10
isbn, RandomIsbn13, random_isbn13
personal, Email, email
personal, FrenchEmail, french_email
personal, FrenchPhoneNumber, french_phone_number
personal, PhoneNumber, phone_number
personal, UkPhoneNumber, uk_phone_number
primitive, AlphanumericCapitalChar, alphanumeric_capital_char
primitive, AlphanumericChar, alphanumeric_char
primitive, Boolean, boolean
primitive, CapitalChar, capital_char
primitive, Digit, digit
primitive, LowerChar, lower_char
;
// cd src/data/raw && ls | while read f; do cat $f | grep '^pub const' | tr ':' ' ' | awk '{print $3}' | while read l; do echo "$f, $(caseify -p $l), $l" | sed 's/\.rs//;s/\( .*s\), /\1, /'; done  ; done
art, LiteraryGenre, LITERARY_GENRES
art, ArchitecturalStyle, ARCHITECTURAL_STYLES
art, MusicalGenre, MUSICAL_GENRES
art, MusicalInstrument, MUSICAL_INSTRUMENTS
colour, ColourName, COLOUR_NAMES
currency, CurrencyName, CURRENCY_NAMES
currency, CurrencySymbol, CURRENCY_SYMBOLS
currency, CurrencyCode, CURRENCY_CODES
datetime, DaysOfWeek, DAYS_OF_WEEK
datetime, Month, MONTHS
datetime, DaysOfWeekAbbr, DAYS_OF_WEEK_ABBR
datetime, MonthsAbbr, MONTHS_ABBR
datetime, TimeZone, TIME_ZONES
datetime, TimeUnit, TIME_UNITS
datetime, AmPm, AM_PM
fauna, Animal, ANIMALS
fauna, Mammal, MAMMALS
fauna, Bird, BIRDS
fauna, Insect, INSECTS
fauna, Fishe, FISHES
fauna, Amphibian, AMPHIBIANS
fauna, Reptile, REPTILES
fauna, Mollusc, MOLLUSC
fauna, AnimalType, ANIMAL_TYPES
france, FrenchStreet, FRENCH_STREETS
france, FrenchRoadType, FRENCH_ROAD_TYPES
france, FrenchCountie, FRENCH_COUNTIES
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
people, Painter, PAINTERS
people, Writer, WRITERS
people, Composer, COMPOSERS
people, Mathematician, MATHEMATICIANS
people, Physician, PHYSICIANS
people, Biologist, BIOLOGISTS
people, ComputerScientist, COMPUTER_SCIENTISTS
people, Philosopher, PHILOSOPHERS
programming, ProgrammingLanguage, PROGRAMMING_LANGUAGES
programming, ProgrammingParadigm, PROGRAMMING_PARADIGMS
programming, EditorColourTheme, EDITOR_COLOUR_THEMES
programming, ItDomain, IT_DOMAINS
science, ChemicalElement, CHEMICAL_ELEMENTS
science, MathematicalFunction, MATHEMATICAL_FUNCTIONS
space, Constellation, CONSTELLATIONS
space, Planet, PLANETS
space, Star, STARS
space, Galaxie, GALAXIES
sport, Sport, SPORTS
uk, UkCountyCode, UK_COUNTY_CODES
uk, UkPostcodeArea, UK_POSTCODE_AREAS
uk, UkCountie, UK_COUNTIES
uk, UkCitie, UK_CITIES
uk, UkRoadType, UK_ROAD_TYPES
uk, LongestUkRiver, LONGEST_UK_RIVERS
uk, UkStreet, UK_STREETS
university, SchoolSubject, SCHOOL_SUBJECTS
university, AcademicDiscipline, ACADEMIC_DISCIPLINES
university, DegreesTitle, DEGREES_TITLES
university, Universitie, UNIVERSITIES
us, UsStateAbbr, US_STATE_ABBR
us, AmericanStates, AMERICAN_STATES
us, UsRoads, US_ROADS
us, UsRoadTypes, US_ROAD_TYPES
weather, TypesOfCloud, TYPES_OF_CLOUDS
work, CarBrand, CAR_BRANDS
work, CompanyName, COMPANY_NAMES
work, Job, JOBS
world, Countrie, COUNTRIES
world, MostPopulatedCitie, MOST_POPULATED_CITIES
world, Continent, CONTINENTS
world, CountryCode, COUNTRY_CODES
world, WorldStreet, WORLD_STREETS
world, LongestRiver, LONGEST_RIVERS
);
