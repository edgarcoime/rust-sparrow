use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub animals: Vec<Animal>
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world
            .animals()
            .iter()
            .map(Animal::from)
            .collect();
        
            Self { animals }
    }
}