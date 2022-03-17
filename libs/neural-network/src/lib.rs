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
}


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Setup Neural-Network Tests
}