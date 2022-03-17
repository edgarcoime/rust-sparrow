#![feature(crate_visibility_modifier)]

use nalgebra as na;
use rand::{Rng, RngCore};
pub use self::{world::*, animal::*, food::*};

mod eye;
mod world;
mod animal;
mod food;

#[derive(Debug)]
pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng)
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Performs a single step - a single frame of our animation
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }
}

impl Simulation {
    fn process_movements(&mut self) {
        self.world.animals.iter_mut()
            .for_each(|a| a.process_movement())
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in self.world.animals.iter_mut() {
            for food in self.world.foods.iter_mut() {
                let distance = na::distance(
                    &animal.position,
                    &food.position
                );

                if distance <= 0.01 {
                    food.position = rng.gen();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use rand::thread_rng;
    use na::{Vector2, Vector3};
    use super::*;

    #[test]
    fn test() {
        let mut rng = thread_rng();
        // A vector with three components.
        let mut my_animal = Animal::random(&mut rng);
        // // let my_vec3 = Vector2::new(0., 1., 0.);
        println!("{:?}", my_animal.position);
        my_animal.process_movement();
        println!("{:?}", my_animal.position);
        // println!("{:?}", my_animal);
        // println!("{}", my_animal.position);
        // println!("{}", my_animal.rotation);
        // let mut vec1 = Vector3::new(1.0, 2.0, 3.0);
    }
}
