use rand::rngs::ThreadRng;
use rand::{RngCore, SeedableRng as _};
use rand_chacha::ChaCha20Rng;

/// Generator provded to the data types. Refer to [`DataType`](crate::DataType) for usage information.
///
/// # Examples
///
/// ```
/// use random_data::DataGenerator;
/// use random_data::rand;
///
/// // Create a default generator
/// let _ = DataGenerator::default();
///
/// // Create a generator with a custom random
/// let _ = DataGenerator::from(rand::rng());
/// ```
pub struct DataGenerator<Rng: RngCore>(Rng);

impl<Rng: RngCore> DataGenerator<Rng> {
    /// Returns the inner random generator
    pub const fn rng(&mut self) -> &mut Rng {
        &mut self.0
    }
}

impl Default for DataGenerator<ThreadRng> {
    fn default() -> Self {
        Self(rand::rng())
    }
}

impl DataGenerator<ChaCha20Rng> {
    /// Returns a [`DataGenerator`] with a given seed
    #[must_use]
    pub fn new_with_seed(seed: u64) -> Self {
        Self(ChaCha20Rng::seed_from_u64(seed))
    }
}

impl<Rng: RngCore> From<Rng> for DataGenerator<Rng> {
    fn from(value: Rng) -> Self {
        Self(value)
    }
}
