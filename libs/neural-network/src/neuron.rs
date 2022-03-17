use rand::Rng;

use crate::*;

// A Neuron contains biases and output weights
#[derive(Debug, Clone)]
pub struct Neuron {
    crate bias: f32,
    crate weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        // Want random bias to simulate randomized evolution
        // 0..3 is exlusive so (0, 1, 2)
        // 0..=3 is inclusive so includes 3
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use super::*;


    macro_rules! assert_almost_eq {
        ($left:expr, $right:expr) => {
            let left: f32 = $left;
            let right: f32 = $right;

            assert!((left - right).abs() < f32::EPSILON)
        };
    }

    #[test]
    fn returns_floating_point_accuracy() {
        // Because we always use the same seed, our `rng` in here will always
        // return the same set of values
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        // https://floating-point-gui.de
        // There can be floating point inaccuracies so in a large amount of cases
        // floating points cannot be asserted as EXACTLY the same. 
        // Therefore we need to ensure that based on the floating point epsilon that the two 
        // floats are relatively close enough to be the same.
        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(neuron.weights.as_slice(), &[
            0.67383957, 
            0.8181262, 
            0.26284897, 
            0.5238807
        ].as_ref());
    }

    #[test]
    fn returns_propagated_result() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // Ensures `.max()` (our ReLU) works:
        approx::assert_relative_eq!(
            neuron.propagate(&[-10., -10.]),
            0.
        );

        // `0.5` and `1.0` chosen by a fair dice roll:
        approx::assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        )
    }
}
