use rand::{Rng, RngCore};


#[derive(Debug)]
pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    pub fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        // Asserts implementation is correct b/c data here is internal
        // if wrong then our implementation is wrong
        assert_eq!(inputs.len(), self.weights.len());

        let ouput = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + ouput).max(0.0)
    }
}
