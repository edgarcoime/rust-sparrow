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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(
                self.fov_range,
                self.fov_angle,
                TEST_EYE_CELLS,
            );

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    match cell {
                        c if c >= 0.7 => "#",
                        c if c >= 0.3 => "+",
                        c if c >  0.0 => ".",
                        _ => " "
                    }
                })
                .collect();
            
            let actual_vision = actual_vision.join("");
            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food { position: na::Point2::new(x, y) }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")]  // Food is inside the FOV
        #[test_case(0.9, "      +      ")]  // ^^
        #[test_case(0.8, "      +      ")]  // ^^
        #[test_case(0.7, "      .      ")]  // Food slowly disappears
        #[test_case(0.6, "      .      ")]  // ^^
        #[test_case(0.5, "             ")]  // Food disappears!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_range,
                expected_vision,
            }.run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         +   ")] // Food is to the right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (Continue to see it
        #[test_case(1.50 * PI, "            +")] // Because of 360% fov angle)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0, 
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }.run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Checking the X axis
        // (You can see the bird "flying away" from the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (You can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }.run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " +         + ")]
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")]
        #[test_case(1.00 * PI, "   . + + .   ")]
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")]
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }.run()
        }
    }
}