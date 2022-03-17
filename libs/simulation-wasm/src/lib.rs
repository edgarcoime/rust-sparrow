pub use self::{animal::*, world::*, food::*};
use serde::Serialize;
use lib_simulation as sim;
use wasm_bindgen::prelude::*;
use rand::prelude::*;

mod animal;
mod food;
mod world;

#[wasm_bindgen]
// #[derive(Debug)]
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

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "That's not a dog that's a turtle".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sim = Simulation::new();
        println!("{:?}", sim.world());
        println!("Hello world");
    }
}