use rand::seq::SliceRandom;
use rand::{Rng, RngCore};
pub struct GeneticAlgorithm;

pub use self::{
    individual::*,
    selection::*,
};

mod individual;
mod selection;

impl GeneticAlgorithm {
    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());
        (0..population.len())
            // TODO selection
            // TODO crossover
            // TODO mutation
            .map(|_| todo!())
            .collect()
    }
}


pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("population is empty")
    }
}
