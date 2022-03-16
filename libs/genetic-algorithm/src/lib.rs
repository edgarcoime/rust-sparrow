pub struct GeneticAlgorithm

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<T>(&self, population: &[T]) -> Vec<T> {
        // https://www.tutorialspoint.com/genetic_algorithms/genetic_algorithms_parent_selection.htm
        // Fitness algorithm to use
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