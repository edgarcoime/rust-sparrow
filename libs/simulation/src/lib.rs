mod world;
mod animal;
mod food;

use nalgebra as na;
use rand::{Rng, RngCore};
use crate::{animal::*, food::*, world::*};


pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}

