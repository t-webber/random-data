use rand::rngs::ThreadRng;

#[derive(Default)]
pub struct DataGenerator(ThreadRng);

impl DataGenerator {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn rng(&mut self) -> &mut ThreadRng {
        &mut self.0
    }
}
