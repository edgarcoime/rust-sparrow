pub use self::roulette_wheel::*;

use crate::*;

mod roulette_wheel;

// TODO: Implement rank based selection
// https://setu677.medium.com/how-to-perform-roulette-wheel-and-rank-based-selection-in-a-genetic-algorithm-d0829a37a189
// Better selection would be rank based selection

pub trait SelectionMethod {
    fn select<'a, I>(
        &self, 
        rng: &mut dyn RngCore,
        population: &'a [I]
    ) -> &'a I
    where
        I: Individual;
}

// TODO: Write tests