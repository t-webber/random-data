mod computational;
mod raw;
extern crate alloc;
use crate::generator::DataGenerator;
use alloc::fmt;

#[allow(unused_imports, reason = "minimal feature doesn't use this")]
use rand::seq::IndexedRandom as _;

macro_rules! strings {
    ($($fn_module:ident, $fn_feature:literal, $fn_variant:ident, $fn_func:ident)*;  $($list_module:ident, $list_feature:literal, $list_variant:ident, $list_const:ident)*) => {


        /// Representation of type that can be generated randomly.
        ///
        /// There are two types of generated data: some are hard-coded with a list of possible
        /// values, others are produced by formulas. Both are usable the same way. If you don't
        /// want to load all the possible values for all the types, you can choose to enable
        /// the data types you want (all features are enabled by default).
        ///
        /// # Examples
        ///
        /// ```
        /// use random_data::*;
        /// let mut generator = DataGenerator::default();
        ///
        /// #[cfg(feature = "datetime")]
        /// {
        ///     let random_month = DataType::Month.random(&mut generator);
        ///     println!("{random_month}");
        /// }
        ///
        /// #[cfg(feature = "address")]
        /// {
        ///     let random_address = DataType::Address.random(&mut generator);
        ///     println!("{random_address}");
        /// }
        /// ```
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
        #[allow(clippy::arbitrary_source_item_ordering, reason="ordered by type")]
        #[allow(missing_docs, reason="macro produced")]
        pub enum DataType {
            $(
                #[cfg(feature = $fn_feature)]
                $fn_variant,
            )*
            $(
                #[cfg(feature = $list_feature)]
                $list_variant,
            )*
        }

        impl DataType {
            const STRINGS_LIST: &[&str] = &[
                $(
                    #[cfg(feature = $fn_feature)]
                    stringify!($fn_variant),
                )*
                $(
                    #[cfg(feature = $list_feature)]
                    stringify!($list_variant),
                )*
            ];

            const TYPES_LIST: &[Self] = &[
                $(
                    #[cfg(feature = $fn_feature)]
                    Self::$fn_variant,
                )*
                $(
                    #[cfg(feature = $list_feature)]
                    Self::$list_variant,
                )*
            ];


            /// Generate a random value of the according [`DataType`]
            ///
            /// # Examples
            ///
            /// ```
            /// use random_data::*;
            /// let mut generator = DataGenerator::default();
            ///
            /// #[cfg(feature = "science")]
            /// {
            ///     let random_address = DataType::ChemicalElement.random(&mut generator);
            ///     println!("{random_address}");
            /// }
            /// ```
            pub fn random(&self, generator: &mut DataGenerator) -> String {
                match self {
                    $(
                        #[cfg(feature = $fn_feature)]
                        DataType::$fn_variant => crate::data::computational::$fn_module::$fn_func(generator),
                    )*
                    $(
                        #[cfg(feature = $list_feature)]
                        DataType::$list_variant => (crate::data::raw::$list_module::$list_const).choose(generator.rng()).unwrap().to_string(),
                    )*
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
            /// let mut generator = DataGenerator::default();
            ///
            /// #[cfg(feature = "people")]
            /// {
            ///     let mathematician = DataType::Mathematician.random(&mut generator);
            ///     let mathematicians = DataType::Mathematician.values().unwrap();
            ///     assert!(mathematicians.contains(&mathematician.as_ref()));
            /// }
            ///
            /// #[cfg(feature = "personal")]
            /// assert!(DataType::Email.values().is_none());
            /// ```
            #[must_use]
            #[allow(unreachable_patterns, reason="features")]
            pub const fn values(&self) -> Option<&'static[&'static str]> {
                match self {
                    $(
                        #[cfg(feature = $list_feature)]
                        Self::$list_variant => Some(&crate::data::raw::$list_module::$list_const),
                    )*
                        _ => None
                }
            }

        }

        impl TryFrom<&str> for DataType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, ()> {
                match value {
                    $(
                        #[cfg(feature = $fn_feature)]
                        stringify!($fn_variant) => Ok(Self::$fn_variant),
                    )*
                    $(
                        #[cfg(feature = $list_feature)]
                        stringify!($list_variant) => Ok(Self::$list_variant),
                    )*
                    _ => Err(())

                }
            }
        }

    };
}

impl DataType {
    /// List of all the available data types.
    ///
    /// # Examples
    ///
    /// ```
    /// use random_data::*;
    ///
    /// #[cfg(feature = "computer")]
    /// assert!(DataType::list().contains(&DataType::DirPath));
    /// ```
    #[must_use]
    pub const fn list() -> &'static [Self] {
        Self::TYPES_LIST
    }

    /// List of all the available data types, in string format.
    ///
    /// # Examples
    ///
    /// ```
    /// use random_data::*;
    ///
    /// #[cfg(feature = "sky_space")]
    /// assert!(DataType::list_str().contains(&"TypesOfCloud"));
    /// ```
    #[must_use]
    pub const fn list_str() -> &'static [&'static str] {
        Self::STRINGS_LIST
    }
}

#[allow(clippy::use_debug, reason = "fine here")]
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
address, "address", Address, address
address, "address", FrenchAddress, french_address
address, "address", FrenchPostCode, french_post_code
address, "address", FrenchStreet, french_street_name
address, "address", Latitude, latitude
address, "address", LatitudeLongitude, latitude_longitude
address, "address", Longitude, longitude
address, "address", StreetNumber, street_number
address, "address", UkAddress, uk_address
address, "address", UkPostCode, uk_post_code
colour, "minimal", HexColour, hex_colour
colour, "minimal", HslaColour, hsla_colour
colour, "minimal", HslColour, hsl_colour
colour, "minimal", RgbaColour, rgba_colour
colour, "minimal", RgbColour, rgb_colour
computer, "computer", DirPath, dir_path
computer, "computer", FileName, file_name
computer, "computer", FilePath, file_path
computer, "computer", Ipv4, ipv4
computer, "computer", Ipv6, ipv6
computer, "computer", MacAddress, mac_address
computer, "computer", Semver, semver
computer, "computer", SemverStable, semver_stable
computer, "computer", SemverUnstable, semver_unstable
finance, "finance", Bic, bic
finance, "finance", Iban, iban
finance, "finance", Isin, isin
isbn, "minimal", RandomIsbn10, random_isbn10
isbn, "minimal", RandomIsbn13, random_isbn13
people, "people", FamousPerson, famous_person
personal, "personal", CreditCard, credit_card
personal, "personal", Email, email
personal, "personal", FrenchEmail, french_email
personal, "personal", FrenchLicencePlate, french_licence_plate
personal, "personal", FrenchPhoneNumber, french_phone_number
personal, "personal", NhsNumber, nhs_number
personal, "personal", Password, password
personal, "personal", PhoneNumber, phone_number
personal, "personal", SecuriteSociale, securite_sociale
personal, "personal", UkLicencePlate, uk_licence_plate
personal, "personal", UkPhoneNumber, uk_phone_number
primitives, "minimal", AlphanumericCapitalChar, alphanumeric_capital_char
primitives, "minimal", AlphanumericChar, alphanumeric_char
primitives, "minimal", Boolean, boolean
primitives, "minimal", CapitalChar, capital_char
primitives, "minimal", Digit, digit
primitives, "minimal", LowerChar, lower_char
primitives, "minimal", Number, number
text, "text", Sentence, sentence
text, "text", Paragraph, paragraph
;
// cd src/data/raw && ls | while read f; do cat $f | grep '^pub const' | tr ':' ' ' | awk '{print $3}' | while read l; do echo "$f, $(caseify -p $l), $l" | sed 's/\.rs//;s/\( .*s\), /\1, /'; done  ; done
animals, "animals", Animal, ANIMALS
animals, "animals", Mammal, MAMMALS
animals, "animals", Bird, BIRDS
animals, "animals", Insect, INSECTS
animals, "animals", Fishe, FISHES
animals, "animals", Amphibian, AMPHIBIANS
animals, "animals", Reptile, REPTILES
animals, "animals", Mollusc, MOLLUSC
animals, "animals", AnimalType, ANIMAL_TYPES
animals, "animals", MythologicalCreature, MYTHOLOGICAL_CREATURES
art, "art", LiteraryGenre, LITERARY_GENRES
art, "art", ArchitecturalStyle, ARCHITECTURAL_STYLES
art, "art", MusicalGenre, MUSICAL_GENRES
art, "art", MusicalInstrument, MUSICAL_INSTRUMENTS
colours, "colours", ColourName, COLOUR_NAMES
currencies, "currencies", CurrencyName, CURRENCY_NAMES
currencies, "currencies", CurrencySymbol, CURRENCY_SYMBOLS
currencies, "currencies", CurrencyCode, CURRENCY_CODES
datetime, "datetime", DaysOfWeek, DAYS_OF_WEEK
datetime, "datetime", Month, MONTHS
datetime, "datetime", DaysOfWeekAbbr, DAYS_OF_WEEK_ABBR
datetime, "datetime", MonthsAbbr, MONTHS_ABBR
datetime, "datetime", TimeZone, TIME_ZONES
datetime, "datetime", TimeUnit, TIME_UNITS
datetime, "datetime", AmPm, AM_PM
education, "education", SchoolSubject, SCHOOL_SUBJECTS
education, "education", AcademicDiscipline, ACADEMIC_DISCIPLINES
education, "education", DegreesTitle, DEGREES_TITLES
education, "education", University, UNIVERSITIES
education, "education", Sport, SPORTS
france, "france", FamousFrenchStreet, FRENCH_STREETS
france, "france", FrenchRoadType, FRENCH_ROAD_TYPES
france, "france", FrenchCounty, FRENCH_COUNTIES
france, "france", FrenchRiver, FRENCH_RIVERS
history, "history", HistoricalBattle, HISTORICAL_BATTLES
internet, "internet", EmailDomain, EMAIL_DOMAINS
internet, "internet", OpenSourceApp, OPEN_SOURCE_APPS
internet, "internet", TopLevelDomain, TOP_LEVEL_DOMAINS
internet, "internet", UserAgent, USER_AGENTS
internet, "internet", HttpStatusCode, HTTP_STATUS_CODES
internet, "internet", MimeType, MIME_TYPES
names, "names", NameTitle, NAME_TITLES
names, "names", FirstName, FIRST_NAMES
names, "names", FrenchFirstName, FRENCH_FIRST_NAMES
names, "names", FrenchLastName, FRENCH_LAST_NAMES
names, "names", LastName, LAST_NAMES
people, "people", Painter, PAINTERS
people, "people", Writer, WRITERS
people, "people", Composer, COMPOSERS
people, "people", Mathematician, MATHEMATICIANS
people, "people", Physician, PHYSICIANS
people, "people", Biologist, BIOLOGISTS
people, "people", ComputerScientist, COMPUTER_SCIENTISTS
people, "people", Philosopher, PHILOSOPHERS
computer, "computer", ProgrammingLanguage, PROGRAMMING_LANGUAGES
computer, "computer", ProgrammingParadigm, PROGRAMMING_PARADIGMS
computer, "computer", EditorColourTheme, EDITOR_COLOUR_THEMES
computer, "computer", ItDomain, IT_DOMAINS
computer, "computer", FileExtension, FILE_EXTENSIONS
science, "science", ChemicalElement, CHEMICAL_ELEMENTS
science, "science", MathematicalFunction, MATHEMATICAL_FUNCTIONS
sky_space, "sky_space", TypesOfCloud, TYPES_OF_CLOUDS
sky_space, "sky_space", Constellation, CONSTELLATIONS
sky_space, "sky_space", Planet, PLANETS
sky_space, "sky_space", Star, STARS
sky_space, "sky_space", Galaxy, GALAXIES
uk, "uk", UkCountyCode, UK_COUNTY_CODES
uk, "uk", UkPostcodeArea, UK_POSTCODE_AREAS
uk, "uk", UkCounty, UK_COUNTIES
uk, "uk", UkCity, UK_CITIES
uk, "uk", UkRoadType, UK_ROAD_TYPES
uk, "uk", UkRiver, UK_RIVERS
uk, "uk", UkStreet, UK_STREETS
us, "us", UsStateAbbr, US_STATE_ABBR
us, "us", UsState, US_STATES
us, "us", UsRoad, US_ROADS
us, "us", UsRoadType, US_ROAD_TYPES
text, "text", Word, WORDS
work, "work", Job, JOBS
work, "work", CompanyName, COMPANY_NAMES
work, "work", ItCompanyName, IT_COMPANY_NAMES
work, "work", EnergyCompanyName, ENERGY_COMPANY_NAMES
work, "work", FinanceCompanyName, FINANCE_COMPANY_NAMES
work, "work", RetailCompanyName, RETAIL_COMPANY_NAMES
work, "work", FoodCompanyName, FOOD_COMPANY_NAMES
work, "work", TravelCompanyName, TRAVEL_COMPANY_NAMES
work, "work", ConstructionCompanyName, CONSTRUCTION_COMPANY_NAMES
work, "work", MediaCompanyName, MEDIA_COMPANY_NAMES
work, "work", TelecomCompanyName, TELECOM_COMPANY_NAMES
work, "work", BioCompanyName, BIO_COMPANY_NAMES
work, "work", CarBrand, CAR_BRANDS
work, "work", AirDefenseCompanyName, AIR_DEFENSE_COMPANY_NAMES
work, "work", ClothingCompanyName, CLOTHING_COMPANY_NAMES
world, "world", Country, COUNTRIES
world, "world", City, CITIES
world, "world", Continent, CONTINENTS
world, "world", CountryCode, COUNTRY_CODES
world, "world", Street, STREETS
world, "world", River, RIVERS
);
