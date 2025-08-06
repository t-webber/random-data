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
    ($($fn_module:ident, $fn_variant:ident, $fn_func:ident)*;  $($list_module:ident, $list_variant:ident, $list_const:ident)*) => {


            /// Representation of type that can be generated randomly.
            ///
            /// There are two types of generated data: some are hard-coded with a list of possible
            /// values, others are produced by formulas. Both are usable the same way:
            ///
            /// ```
            /// use random_data::*;
            /// let mut g = DataGenerator::new();
            ///
            /// let random_month = DataType::Month.random(&mut g);
            /// println!("{random_month}");
            ///
            /// let random_address = DataType::Address.random(&mut g);
            /// println!("{random_address}");
            /// ```
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        pub enum DataType {
            $($fn_variant,)*
            $($list_variant,)*
        }

        impl DataType {
            /// List of all the available data types.
            const LIST: &[Self] = &[
                $(Self::$fn_variant,)*
                $(Self::$list_variant,)*
            ];

            /// Returns the list of possible values of a data type if applicable
            ///
            /// # Returns
            ///
            /// - Some if the data type is defined by a list of values
            /// - None if it is generated from a formula
            ///
            /// # Examples
            ///
            /// ```
            /// use random_data::*;
            /// let mut g = DataGenerator::new();
            ///
            /// let month = DataType::Month.random(&mut g);
            /// let all_months = DataType::Month.values().unwrap();
            /// assert!(all_months.contains(&month.as_ref()));
            ///
            /// assert!(DataType::Address.values().is_none());
            /// ```
            pub fn values(&self) -> Option<&'static[&'static str]> {
                match self {
                    $( Self::$list_variant => Some(&$list_module::$list_const),)*
                        _ => None
                }
            }

            /// Generate a random value of the according [`DataType`]
            ///
            /// # Examples
            ///
            /// ```
            /// use random_data::*;
            /// let mut g = DataGenerator::new();
            ///
            /// let random_address = DataType::Address.random(&mut g);
            /// println!("{random_address}");
            /// ```
            pub fn random(&self, g: &mut DataGenerator) -> String {
                match self {
                    $( DataType::$fn_variant => $fn_module::$fn_func(g), )*
                    $( DataType::$list_variant => ($list_module::$list_const).choose(g.rng()).unwrap().to_string(),)*
                }
            }
        }

        impl TryFrom<&str> for DataType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, ()> {
                match value {
                    $( stringify!($fn_variant) => Ok(Self::$fn_variant), )*
                    $( stringify!($list_variant) => Ok(Self::$list_variant), )*
                    _ => Err(())

                }
            }
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
computer, Semver, semver
computer, SemverStable, semver_stable
computer, SemverUnstable, semver_unstable
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
france, FrenchCounty, FRENCH_COUNTIES
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
name, NameTitle, NAME_TITLES
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
space, Galaxy, GALAXIES
sport, Sport, SPORTS
uk, UkCountyCode, UK_COUNTY_CODES
uk, UkPostcodeArea, UK_POSTCODE_AREAS
uk, UkCounty, UK_COUNTIES
uk, UkCity, UK_CITIES
uk, UkRoadType, UK_ROAD_TYPES
uk, LongestUkRiver, LONGEST_UK_RIVERS
uk, UkStreet, UK_STREETS
university, SchoolSubject, SCHOOL_SUBJECTS
university, AcademicDiscipline, ACADEMIC_DISCIPLINES
university, DegreesTitle, DEGREES_TITLES
university, University, UNIVERSITIES
us, UsStateAbbr, US_STATE_ABBR
us, AmericanState, AMERICAN_STATES
us, UsRoads, US_ROADS
us, UsRoadTypes, US_ROAD_TYPES
weather, TypesOfCloud, TYPES_OF_CLOUDS
work, CarBrand, CAR_BRANDS
work, CompanyName, COMPANY_NAMES
work, Job, JOBS
world, Country, COUNTRIES
world, City, CITIES
world, Continent, CONTINENTS
world, CountryCode, COUNTRY_CODES
world, Street, STREETS
world, LongestRiver, LONGEST_RIVERS
);
