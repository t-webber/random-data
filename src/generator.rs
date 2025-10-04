use rand::rngs::ThreadRng;

/// Generator provded to the data types. Refer to [`DataType`](crate::DataType) for usage information.
///
/// # Examples
///
/// ```
/// use random_data::DataGenerator;
/// use rand;
///
/// // Create a default generator
/// let _ = DataGenerator::default();
///
/// // Create a generator with a custom random
/// let _ = DataGenerator::from(rand::rng());
/// ```
#[derive(Default)]
pub struct DataGenerator(ThreadRng);

impl DataGenerator {
    pub(crate) const fn rng(&mut self) -> &mut ThreadRng {
        &mut self.0
    }
}

impl From<ThreadRng> for DataGenerator {
    fn from(value: ThreadRng) -> Self {
        Self(value)
    }
}
