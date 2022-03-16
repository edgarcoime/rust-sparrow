#![feature(type_alias_impl_trait)]

use rand::RngCore;
use self::{individual::*, selection::*, chromosome::*, crossover::*};

mod individual;
mod chromosome;
mod selection;
mod crossover;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(selection_method: S) -> Self {
        Self {
            selection_method
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
                    .select(rng, population);
                
                let parent_b = self
                    .selection_method
                    .select(rng, population);

                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect()
    }
}