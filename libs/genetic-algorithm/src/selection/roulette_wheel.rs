use rand::prelude::SliceRandom;

use crate::*;

// One of the manyy selection methods we could choose
// https://www.tutorialspoint.com/genetic_algorithms/genetic_algorithms_parent_selection.htm

#[derive(Clone, Copy)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I],
    ) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

// TODO: Write tests
#[cfg(test)]
mod test {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test() {
        todo!()
        // let mut rng = ChaCha8Rng::from_seed(Default::default());

        // let population = vec![];

        // let actual = RouletteWheelSelection::new()
        //     .select(&mut rng, &population);

        // assert!()
    }
}
