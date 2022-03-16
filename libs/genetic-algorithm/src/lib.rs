#![feature(type_alias_impl_trait)]

use rand::RngCore;
use self::{individual::*, selection::*, chromosome::*, crossover::*, mutation::*};

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

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual
    {
        assert!(!population.is_empty());

        (0..population.len())
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
                todo!()
            })
            .collect()
    }
}