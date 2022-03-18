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
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        let config: sim::Config = config.into_serde().unwrap();

        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> JsValue {
        JsValue::from_serde(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        JsValue::from_serde(self.sim.config()).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) -> Option<String> {
        self.sim.step(&mut self.rng).map(|stats| stats.to_string())
    }

    pub fn train(&mut self) -> String {
        self.sim.train(&mut self.rng).to_string()
    }
}

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "That's not a dog that's a turtle".into()
}