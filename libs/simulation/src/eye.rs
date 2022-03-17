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
        let mut cells = vec![0.; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            // Food is out of range
            if dist >= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(
                &na::Vector2::x(),
                &vec,
            ).angle();

            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            // if current angle is outside our birdie's fov, jump to the next food
            if  angle < -self.fov_angle / 2. ||
                angle > self.fov_angle / 2.
                {
                    continue
                }
            
            let angle = angle + self.fov_angle / 2.;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
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
