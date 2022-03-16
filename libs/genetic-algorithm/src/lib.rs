#![feature(type_alias_impl_trait)]

use rand::RngCore;
use self::{individual::*, selection::*, chromosome::*, crossover::*};

mod individual;
mod chromosome;
mod selection;
mod crossover;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossOverMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossOverMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method)
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
                // TODO: convert chromosome back into individual
                todo!()
            })
            .collect()
    }
}