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

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self::new(neurons)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Setup Layer Tests

    mod from_weights {
        use super::*;

        #[test]
        fn test() {
            let layer = Layer::from_weights(
                3,
                2,
                &mut vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8].into_iter(),
            );

            let actual_biases: Vec<_> = layer.neurons.iter().map(|neuron| neuron.bias).collect();
            let expected_biases = vec![0.1, 0.5];

            let actual_weights: Vec<_> = layer
                .neurons
                .iter()
                .map(|neuron| neuron.weights.as_slice())
                .collect();
            let expected_weights: Vec<&[f32]> = vec![&[0.2, 0.3, 0.4], &[0.6, 0.7, 0.8]];

            approx::assert_relative_eq!(actual_biases.as_slice(), expected_biases.as_slice());
            approx::assert_relative_eq!(actual_weights.as_slice(), expected_weights.as_slice());
        }
    }
}