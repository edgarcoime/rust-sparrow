pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                todo!()
            })
            .collect()
    }
}
