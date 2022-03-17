use crate::*;

#[derive(Debug)]
pub struct Animal {
    crate position: na::Point2<f32>,
    // TODO: Merge rotation and speed into VELOCITY
    crate rotation: na::Rotation2<f32>,
    crate speed: f32,
    crate eye: Eye,
    crate brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
                // Input layer
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                // Hidden layer
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },
                // Output Layer
                nn::LayerTopology {
                    neurons: 2
                },
            ],
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Animal {
    crate fn process_movement(&mut self) {
        self.position += self.rotation * na::Vector2::new(self.speed, 0.);

        // Wraps animals around
        self.position.x = na::wrap(self.position.x, 0., 1.);
        self.position.y = na::wrap(self.position.y, 0., 1.);
    }
}