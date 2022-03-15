#[cfg(test)]
use super::*;
use approx::assert_relative_eq;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

macro_rules! assert_almost_eq {
    ($left:expr, $right:expr) => {
        let left: f32 = $left;
        let right: f32 = $right;

        assert!((left - right).abs() < f32::EPSILON)
    };
}

mod random {
    use super::*;

    #[test]
    fn test() {
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
}

mod propagate {
    use super::*;

    #[test]
    fn test() {
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