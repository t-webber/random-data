mod computational;
mod raw;
extern crate alloc;
use crate::generator::DataGenerator;
use alloc::fmt;
use rand::seq::IndexedRandom as _;

macro_rules! strings {
    ($($fn_module:ident, $fn_variant:ident, $fn_func:ident)*;  $($list_module:ident, $list_variant:ident, $list_const:ident)*) => {


            /// Representation of type that can be generated randomly.
            ///
            /// There are two types of generated data: some are hard-coded with a list of possible
            /// values, others are produced by formulas. Both are usable the same way:
            ///
            /// ```
            /// use random_data::*;
            /// let mut generator = DataGenerator::new();
            ///
            /// let random_month = DataType::Month.random(&mut generator);
            /// println!("{random_month}");
            ///
            /// let random_address = DataType::Address.random(&mut generator);
            /// println!("{random_address}");
            /// ```
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        #[expect(clippy::arbitrary_source_item_ordering, reason="ordered by type")]
        #[expect(missing_docs, reason="macro produced")]
        pub enum DataType {
            $($fn_variant,)*
            $($list_variant,)*
        }

        impl DataType {
            const STRINGS_LIST: &[&str] = &[
                $(stringify!($fn_variant),)*
                $(stringify!($list_variant),)*
            ];

            const TYPES_LIST: &[Self] = &[
                $(Self::$fn_variant,)*
                $(Self::$list_variant,)*
            ];


            /// Generate a random value of the according [`DataType`]
            ///
            /// # Examples
            ///
            /// ```
            /// use random_data::*;
            /// let mut generator = DataGenerator::new();
            ///
            /// let random_address = DataType::Address.random(&mut generator);
            /// println!("{random_address}");
            /// ```
            pub fn random(&self, generator: &mut DataGenerator) -> String {
                match self {
                    $( DataType::$fn_variant => crate::data::computational::$fn_module::$fn_func(generator), )*
                    $( DataType::$list_variant => (crate::data::raw::$list_module::$list_const).choose(generator.rng()).unwrap().to_string(),)*
                }
            }

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
            /// let mut generator = DataGenerator::new();
            ///
            /// let month = DataType::Month.random(&mut generator);
            /// let all_months = DataType::Month.values().unwrap();
            /// assert!(all_months.contains(&month.as_ref()));
            ///
            /// assert!(DataType::Address.values().is_none());
            /// ```
            #[must_use]
            pub const fn values(&self) -> Option<&'static[&'static str]> {
                match self {
                    $( Self::$list_variant => Some(&crate::data::raw::$list_module::$list_const),)*
                        _ => None
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
    /// List of all the available data types.
    #[must_use]
    pub const fn list() -> &'static [Self] {
        Self::TYPES_LIST
    }

    /// List of all the available data types, in string format.
    #[must_use]
    pub const fn list_str() -> &'static [&'static str] {
        Self::STRINGS_LIST
    }
}

#[expect(clippy::use_debug, reason = "fine here")]
#[expect(clippy::min_ident_chars, reason = "follow trait naming patterns")]
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
colour, HexColour, hex_colour
colour, HslaColour, hsla_colour
colour, HslColour, hsl_colour
colour, RgbaColour, rgba_colour
colour, RgbColour, rgb_colour
computer, DirPath, dir_path
computer, FileName, file_name
computer, FilePath, file_path
computer, Ipv4, ipv4
computer, Ipv6, ipv6
computer, MacAddress, mac_address
computer, Semver, semver
computer, SemverStable, semver_stable
computer, SemverUnstable, semver_unstable
finance, Bic, bic
finance, Iban, iban
finance, Isin, isin
isbn, RandomIsbn10, random_isbn10
isbn, RandomIsbn13, random_isbn13
personal, CreditCard, credit_card
personal, Email, email
personal, FrenchEmail, french_email
personal, FrenchLicencePlate, french_licence_plate
personal, FrenchPhoneNumber, french_phone_number
personal, NhsNumber, nhs_number
personal, Password, password
personal, PhoneNumber, phone_number
personal, SecuriteSociale, securite_sociale
personal, Sentence, sentence
personal, Paragraph, paragraph
personal, UkLicencePlate, uk_licence_plate
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
fauna, Fish, FISHES
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
internet, HttpStatusCode, HTTP_STATUS_CODES
internet, MimeType, MIME_TYPES
internet, OpenSourceApp, OPEN_SOURCE_APPS
internet, TopLevelDomain, TOP_LEVEL_DOMAINS
internet, UserAgent, USER_AGENTS
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
programming, FileExtension, FILE_EXTENSIONS
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
words, Word, WORDS
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
