#![feature(crate_visibility_modifier)]

pub use self::{world::*, animal::*, food::*, eye::*, brain::*};
use animal_individual::AnimalIndividual;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;

mod eye;
mod world;
mod animal;
mod animal_individual;
mod food;
mod brain;

// region:      Constants

/// Mimum speed of bird
const SPEED_MIN: f32 = 0.001;
/// Maximum speed of bird
const SPEED_MAX: f32 = 0.005;
/// Acceleration of speed of bird
const SPEED_ACCEL: f32 = 0.2;
/// Rotation Acceleration
const ROTATION_ACCEL: f32 = FRAC_PI_2;

// How many steps each bird gets to live 
const GENERATIONAL_LENGTH: usize = 2500;

// endregion:   Constants

pub struct Simulation {
    world: World,
    ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlgorithm::new (
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(0.01, 0.3),
            // Higher values make sim more chaotic which may allow for better solutions;
            // but trade-off is that higher values cause good enough solutions currently
            // to be discarded
        );

        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    /// Performs a single step - a single frame of our animation
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATIONAL_LENGTH {
            self.evolve(rng);
        }
    }
}

impl Simulation {
    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Step 1: Prepare animals to be sent into genetic algo.
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Step 2: Evolve animals
        let evolved_population = self.ga.evolve(rng, &current_population);

        // Step 3: Bring animals back from the genetic algo
        self.world.animals = todo!();

        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // Step 4: Restart foods
        for food in self.world.foods.iter_mut() {
            food.position = rng.gen();
        }
    }

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
                    animal.satiation += 1;
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
