use crate::*;
use std::f32::consts::*;

/// How far our eye can see:
/// - 0.1 = 10% of the map = bird sees no foods (at least in this case)
/// - 0.5 = 50% of the map = bird sees one of the foods
/// - 1.0 = 100% of the map = bird sees both foods
const FOV_RANGE: f32 = 0.25;

/// How wide our eye can see:
/// If @> marks our birdie (rotated to the right) and . marks the area
/// our birdie sees, then a FOV_ANGLE of:
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

/// How many photoreceptors there are in a single eye
const DEFAULT_CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        todo!()
    }
}

impl Eye {
    fn new (fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.);
        assert!(fov_angle > 0.);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, DEFAULT_CELLS)
    }
}
