use super::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn test() {
    // Because we always use the same seed, our `rng` in here will always
    // return the same set of values
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let neuron = Neuron::random(&mut rng, 4);

    assert_eq!(neuron.bias, -0.6255188);
    assert_eq!(neuron.weights, &[
        0.67383957, 
        0.8181262, 
        0.26284897, 
        0.5238807
    ]);
}