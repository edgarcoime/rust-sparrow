// A network is built from layers
pub struct Network {
    layers: Vec<Layer>
}

// A layer is built from neurons
struct Layer {
    neurons: Vec<Neuron>
}

// A Neuron contains biases and output weights
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}


impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
