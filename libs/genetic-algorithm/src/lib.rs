#![feature(type_alias_impl_trait)]
#![feature(crate_visibility_modifier)]
use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
pub use self:: {
    chromosome::*,
    individual::*,
    crossover::*,
    mutation::*,
    selection::*,
    statistics::*,
};

mod statistics;
mod individual;
mod chromosome;
mod selection;
mod crossover;
mod mutation;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossOverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossOverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, Statistics)
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        let new_population = (0..population.len())
            .map(|_| {
                // TODO: selection
                let parent_a = self
                    .selection_method
                    .select(rng, population).chromosome();
                
                let parent_b = self
                    .selection_method
                    .select(rng, population).chromosome();

                // TODO: crossover
                let mut child = self
                    .crossover_method
                    .crossover(rng, parent_a, parent_b);

                // TODO: mutation
                self.mutation_method.mutate(rng, &mut child);

                // TODO: convert chromosome back into individual
                I::create(child)
            })
            .collect();
        
        (new_population, Statistics::new(population))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();
        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0., 0., 0.]),  // fitness = 0.0
            individual(&[1., 1., 1.]),  // fitness = 3.0
            individual(&[1., 2., 1.]),  // fitness = 4.0
            individual(&[1., 2., 4.]),  // fitness = 7.0
        ];

        // Running `.evolve()` so that differences between initial and ouput popolation are easier to spot
        for _ in 0..25 {
            population = ga.evolve(&mut rng, &population).0;
        }

        let expected_population = vec![
            individual(&[0.109105855, 2.796503, 3.5660858]),
            individual(&[0.9319788, 2.9991293, 4.931279]),
            individual(&[0.017960489, 3.0900886, 4.1272745]),
            individual(&[-0.3720149, 2.6197953, 4.113259]),
        ];

        assert_eq!(population, expected_population);
    }
}