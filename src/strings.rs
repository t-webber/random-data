pub mod animals;
pub mod art;
pub mod colours;
pub mod companies;
pub mod currency;
pub mod datetime;
pub mod france;
pub mod history;
pub mod internet;
pub mod mythology;
pub mod names;
pub mod people;
pub mod programming;
pub mod science;
pub mod space;
pub mod sport;
pub mod uk;
pub mod university;
pub mod us;
pub mod weather;
pub mod world;

use rand::rngs::ThreadRng;

#[derive(Default)]
pub struct DataGenerator(ThreadRng);

impl DataGenerator {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

macro_rules! strings {
    ($($module:ident, $variant:ident, $const:ident)*) => {

        use std::fmt;
        use rand::seq::SliceRandom;
        use rand::seq::IndexedRandom;

        #[non_exhaustive]
        pub enum DataType {
            $($variant,)*
        }

        impl fmt::Display for DataType {
            fn fmt(&self, f :&mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant => stringify!($variant).fmt(f),)*
                }
            }
        }

        impl TryFrom<&str> for DataType {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, ()> {
                match value {
                    $( stringify!($variant) => Ok(Self::$variant), )*
                    _ => Err(())

                }
            }

        }

        impl DataGenerator {
            pub fn random(&mut self, data_type: DataType) -> &'static str {
                match data_type {
                    $( DataType::$variant => (crate::strings::$module::$const).choose(&mut self.0).unwrap(),)* }
            }
        }
    };
}

strings!(
animals, CommonAnimals, COMMON_ANIMALS
animals, Mammals, MAMMALS
animals, Birds, BIRDS
animals, Insects, INSECTS
animals, Fishes, FISHES
animals, Amphibians, AMPHIBIANS
animals, Reptiles, REPTILES
animals, Arachnids, ARACHNIDS
animals, Mollusks, MOLLUSKS
animals, Crustaceans, CRUSTACEANS
animals, AnimalTypes, ANIMAL_TYPES
art, LiteraryGenres, LITERARY_GENRES
art, ArchitecturalStyles, ARCHITECTURAL_STYLES
art, MusicalGenres, MUSICAL_GENRES
art, MusicalInstruments, MUSICAL_INSTRUMENTS
colours, ColorNames, COLOR_NAMES
companies, CarBrands, CAR_BRANDS
companies, CompanyNames, COMPANY_NAMES
companies, Jobs, JOBS
currency, CurrencySymbols, CURRENCY_SYMBOLS
currency, CurrencyNames, CURRENCY_NAMES
currency, CurrencyCodes, CURRENCY_CODES
datetime, DaysOfWeek, DAYS_OF_WEEK
datetime, Months, MONTHS
datetime, DaysOfWeekAbbr, DAYS_OF_WEEK_ABBR
datetime, MonthsAbbr, MONTHS_ABBR
france, FrenchStreets, FRENCH_STREETS
france, FrenchRoadTypes, FRENCH_ROAD_TYPES
france, FrenchCounties, FRENCH_COUNTIES
france, LongestFrenchRivers, LONGEST_FRENCH_RIVERS
history, HistoricalBattles, HISTORICAL_BATTLES
internet, EmailDomains, EMAIL_DOMAINS
internet, OpenSourceApps, OPEN_SOURCE_APPS
mythology, MythologicalCreatures, MYTHOLOGICAL_CREATURES
names, Firstnames, FIRSTNAMES
names, FrenchFirstnames, FRENCH_FIRSTNAMES
names, FrenchLastnames, FRENCH_LASTNAMES
names, EnglishLastnames, ENGLISH_LASTNAMES
people, FamousPhilosophers, FAMOUS_PHILOSOPHERS
people, FamousPainters, FAMOUS_PAINTERS
people, FamousWriters, FAMOUS_WRITERS
people, FamousComposers, FAMOUS_COMPOSERS
people, FamousMathematicians, FAMOUS_MATHEMATICIANS
people, FamousPhysicians, FAMOUS_PHYSICIANS
people, FamousBiologists, FAMOUS_BIOLOGISTS
people, FamousComputerScientists, FAMOUS_COMPUTER_SCIENTISTS
programming, ProgrammingLanguages, PROGRAMMING_LANGUAGES
programming, ProgrammingParadigms, PROGRAMMING_PARADIGMS
programming, EditorColorThemes, EDITOR_COLOR_THEMES
programming, ItDomains, IT_DOMAINS
science, ChemicalElements, CHEMICAL_ELEMENTS
science, MathematicalFunctions, MATHEMATICAL_FUNCTIONS
space, Constellations, CONSTELLATIONS
space, Planets, PLANETS
space, Stars, STARS
space, Galaxies, GALAXIES
sport, Sports, SPORTS
datetime, TimeZones, TIME_ZONES
datetime, TimeUnits, TIME_UNITS
datetime, AmPm, AM_PM
uk, UkCountyCodes, UK_COUNTY_CODES
uk, UkCounties, UK_COUNTIES
uk, UkRoadTypes, UK_ROAD_TYPES
uk, LongestUkRivers, LONGEST_UK_RIVERS
uk, UkStreets, UK_STREETS
university, SchoolSubjects, SCHOOL_SUBJECTS
university, AcademicDisciplines, ACADEMIC_DISCIPLINES
university, DegreesTitles, DEGREES_TITLES
university, FamousUniversities, FAMOUS_UNIVERSITIES
us, UsStateAbbreviations, US_STATE_ABBREVIATIONS
us, AmericanStates, AMERICAN_STATES
us, UsRoads, US_ROADS
us, UsRoadTypes, US_ROAD_TYPES
weather, TypesOfClouds, TYPES_OF_CLOUDS
world, Countries, COUNTRIES
world, MostPopulatedCities, MOST_POPULATED_CITIES
world, Continents, CONTINENTS
world, CountryCodes, COUNTRY_CODES
world, WorldStreets, WORLD_STREETS
world, LongestRivers, LONGEST_RIVERS
);
