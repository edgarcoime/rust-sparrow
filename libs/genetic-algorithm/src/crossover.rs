use crate::*;

mod uniform;

// Crossover algorithms to use for 
// combining 2 genomes from parents

pub trait CrossOverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}