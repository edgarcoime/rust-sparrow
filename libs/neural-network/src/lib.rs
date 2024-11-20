use self::{layer::Layer, neuron::Neuron};

mod layer;
mod network;
mod neuron;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
    }

    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };

        // ensure `.max()` (our ReLU) works:
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

        // `0.5` and `1.0` is a fair dice roll
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (0.5 * -0.3) + (1.0 * 0.8) + 0.5,
        );
    }
}
