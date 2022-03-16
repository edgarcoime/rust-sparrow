use rand::RngCore;
use self::{individual::*, selection::*};

mod individual;
mod selection;

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        todo!()
    }
}