#![feature(crate_visibility_modifier)]

pub use self::{world::*, animal::*, food::*, eye::*, brain::*, config::*, animal_individual::*, statistics::*};

mod eye;
mod world;
mod animal;
mod animal_individual;
mod food;
mod brain;
mod config;
mod statistics;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::*;
use serde::{Deserialize, Serialize};

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
    config: Config,
    world: World,
    generation: usize,
    age: usize,
}

impl Simulation {
    pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
        let world = World::random(&config, rng);

        Self {
            config,
            world,
            age: 0,
            generation: 0
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Forcefully evolve population to next generation
    /// Fast-forwards until the end of the current generation
    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(statistics) = self.step(rng) {
                return statistics;
            }
        }
    }

    /// Performs a single step - a single frame of our animation
    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.try_evolving(rng)
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

            let response = animal.brain.nn.propagate(vision);

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

    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age += 1;

        if self.age > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        self.generation += 1;

        // Step 1: Prepare animals to be sent into genetic algo.
        let mut current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        if self.config.ga_reverse == 1 {
            let max_satiation = self
                .world
                .animals
                .iter()
                .map(|animal| animal.satiation)
                .max()
                .unwrap_or_default();
            
            for individual in current_population.iter_mut() {
                individual.fitness = (max_satiation as f32) - individual.fitness;
            }
        }

        // Step 2: Evolve animals
        // TODO: set genetic algo in config?
        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(self.config.ga_mut_chance, self.config.ga_mut_coeff),
        );

        let (evolved_population, statistics) = ga.evolve(rng, &current_population);

        // Step 3: Bring animals back from the genetic algo
        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(&self.config, rng))
            .collect();

        // Step 4: Restart foods
        for food in self.world.foods.iter_mut() {
            food.position = rng.gen();
        }

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
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
    }
}
