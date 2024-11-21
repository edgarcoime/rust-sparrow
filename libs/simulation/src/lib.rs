pub use self::{food::*, animal::*, world::*};

mod world;
mod animal;
mod food;

pub use crate::{animal::*, food::*, world::*};

use nalgebra as na;
use rand::{Rng, RngCore};


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

