use crate::*;

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    // TODO: Merge rotation and speed into VELOCITY
    rotation: na::Rotation2<f32>,
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
        }
    }
}
