pub struct GeneticAlgorithm

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<T>(&self, population: &[T]) -> Vec<T> {
        assert!(!population.is_empty())

        (0..population.len())
            .map(|_| {
                // TODO: Selection
                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect()
    }
}