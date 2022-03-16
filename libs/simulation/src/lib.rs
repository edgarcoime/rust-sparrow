use nalgebra as na;
use rand::{Rng, RngCore};
use self::{world::*, animal::*, food::*};

mod world;
mod animal;
mod food;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }
}
