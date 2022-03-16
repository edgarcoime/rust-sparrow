use rand::Rng;
use crate::*;

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossOverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());

        let parent_a = parent_a.iter();
        let parent_b = parent_b.iter();

        // use combinator instead
        parent_a
            .zip(parent_b)
            .map(|(&a, &b)| if rng.gen_bool(0.5) { a } else { b })
            .collect()
    }
}