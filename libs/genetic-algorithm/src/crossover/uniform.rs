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
        parent_a: &crate::chromosome::Chromosome,
        parent_b: &crate::chromosome::Chromosome,
    ) -> Chromosome {
        let mut child: Vec<f32> = Vec::new(); // New chromosome
        let gene_count = parent_a.len();

        for gene_idx in 0..gene_count {
            let gene = if rng.gen_bool(0.5) {
                parent_a[gene_idx]
            } else {
                parent_b[gene_idx]
            };
        }

        child.into_iter().collect()
    }
}