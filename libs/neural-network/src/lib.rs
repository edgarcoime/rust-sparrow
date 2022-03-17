#![feature(crate_visibility_modifier)]

pub use self::layer_topology::*;
use self::{layer::*, neuron::*};
use rand::{Rng, RngCore};

mod errors;
mod layer;
mod neuron;
mod layer_topology;

// A network is built from layers
#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>
}

impl Network {
    crate fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(
                rng, 
                layers[0].neurons, 
                layers[1].neurons)
            )
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> Vec<f32> {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Setup Neural-Network Tests
    mod weights {
        use super::*;

        #[test]
        fn test() {
            let network = Network::new(vec![
                Layer::new(vec![Neuron::new(0.1, vec![0.2, 0.3, 0.4])]),
                Layer::new(vec![Neuron::new(0.5, vec![0.6, 0.7, 0.8])]),
            ]);

            let actual = network.weights();
            let expected: Vec<f32> = vec![0.1, 0.2];

            approx::assert_relative_eq!(
                actual.as_slice(),
                expected.as_slice(),
            );
        }
    }
}