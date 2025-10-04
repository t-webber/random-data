use rand::rngs::ThreadRng;

/// Generator provded to the data types. Refer to [`DataType`](crate::DataType) for usage information.
#[derive(Default)]
pub struct DataGenerator(ThreadRng);

impl DataGenerator {
    pub(crate) const fn rng(&mut self) -> &mut ThreadRng {
        &mut self.0
    }
}
