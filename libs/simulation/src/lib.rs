#![feature(crate_visibility_modifier)]

pub use self::{world::*, animal::*, food::*, eye::*};
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

mod eye;
mod world;
mod animal;
mod food;

// region:      Constants

/// Mimum speed of bird
const SPEED_MIN: f32 = 0.001;
/// Maximum speed of bird
const SPEED_MAX: f32 = 0.005;
/// Acceleration of speed of bird
const SPEED_ACCEL: f32 = 0.2;
/// Rotation Acceleration
const ROTATION_ACCEL: f32 = FRAC_PI_2;

// endregion:   Constants

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
        self.process_brains();
        self.process_movements();
    }
}

impl Simulation {
    fn process_movements(&mut self) {
        self.world.animals.iter_mut()
            .for_each(|a| a.process_movement())
    }

    fn process_brains(&mut self) {
        for animal in self.world.animals.iter_mut() {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.foods,
            );

            let response = animal.brain.propagate(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = 
                (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(
                animal.rotation.angle() + rotation,
            );
        }
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
