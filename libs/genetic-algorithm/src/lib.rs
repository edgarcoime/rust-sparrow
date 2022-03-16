pub struct GeneticAlgorithm

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<T>(&self, population: &[T]) -> Vec<T> {
        // https://www.tutorialspoint.com/genetic_algorithms/genetic_algorithms_parent_selection.htm
        // Fitness algorithm to use
        // TODO: Implement rank based selection
        // https://setu677.medium.com/how-to-perform-roulette-wheel-and-rank-based-selection-in-a-genetic-algorithm-d0829a37a189
        // Better selection would be rank based selection
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