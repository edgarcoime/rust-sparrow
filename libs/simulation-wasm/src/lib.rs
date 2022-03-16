use lib_simulation as sim;
use wasm_bindgen::prelude::*;
use rand::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }
}

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "That's not a dog that's a turtle".into()
}