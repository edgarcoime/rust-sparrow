// Cannot implement Individual w/o knowing implementation of chromosome
// So create seperate struct instead

use crate::*;

pub struct AnimalIndividual {
    crate fitness: f32,
    crate chromosome: ga::Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        // Fitness should be determined by amount of food eaten
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome()
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}