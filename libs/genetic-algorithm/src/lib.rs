use rand::RngCore;
use self::{individual::*, selection::*};

mod individual;
mod selection;

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

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual
    {
        assert!(!population.is_empty());

        let rng = rand
        
        (0..population.len())
            .map(|_| {
                // TODO: selection
                let parent_a = self
                    .selection_method
                    .select(population);
                
                let parent_b = self
                    .selection_method
                    .select(population);

                // TODO: crossover
                // TODO: mutation
                todo!()
            })
            .collect()
    }
}