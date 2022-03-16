use crate::*;

// A layer is built from neurons
#[derive(Clone, Debug)]
pub struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let mut rng = rand::thread_rng();
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rng, input_neurons))
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