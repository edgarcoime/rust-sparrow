use nalgebra as na;
use self::{world::*, animal::*, food::*};

mod world;
mod animal;
mod food;

pub struct Simulation {
    world: World,
}
