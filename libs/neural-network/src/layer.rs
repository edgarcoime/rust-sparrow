use crate::*;

// A layer is built from neurons
#[derive(Clone, Debug)]
pub struct Layer {
    crate neurons: Vec<Neuron>
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());

        assert!(
            neurons
                .iter()
                .all(|neuron| neuron.weights.len() == neurons[0].weights.len())
        );

        Self { neurons }
    }

    pub fn random(rng: &mut dyn RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Setup Layer Tests
}