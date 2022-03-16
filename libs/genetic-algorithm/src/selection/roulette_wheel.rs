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
mod tests {
    use std::collections::BTreeMap;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use super::*;

    #[test]
    fn probability_distribution() {
        let method = RouletteWheelSelection::new();
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let population = vec![
            TestIndividual::new(2.),
            TestIndividual::new(1.),
            TestIndividual::new(4.),
            TestIndividual::new(5.),
            TestIndividual::new(3.),
        ];

        let actual_histogram: BTreeMap<i32, _> = (0..1000)
            .map(|_| method.select(&mut rng, &population))
            .fold(Default::default(), |mut histogram, individual| {
                *histogram
                    .entry(individual.fitness() as _)
                    .or_default() += 1;
                
                histogram
            });

        let expected_histogram = maplit::btreemap! {
            // fitness => how many times this fitess has been chosen
            1 => 60,
            2 => 142,
            3 => 183,
            4 => 273,
            5 => 342,
        };

        assert_eq!(actual_histogram, expected_histogram);
    }
}
