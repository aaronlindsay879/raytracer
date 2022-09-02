use rand::{distributions::Uniform, prelude::Distribution};

use crate::{colour::Colour, vector::Vector};

pub struct Light {
    pub centre: Vector,
    pub radius: f64,
    pub diffuse_intensity: Colour,
    pub specular_intensity: Colour,

    random: Uniform<f64>,
}

impl Light {
    pub fn new(
        point: Vector,
        radius: f64,
        diffuse_intensity: Colour,
        specular_intensity: Colour,
    ) -> Self {
        Self {
            centre: point,
            radius,
            diffuse_intensity,
            specular_intensity,
            random: Uniform::from(-radius..radius),
        }
    }

    /// Simple helper function to check if a point is within the light
    fn point_in_light(&self, vector: Vector) -> bool {
        (self.centre - vector).magnitude() <= self.radius
    }

    /// Generate a random point within the radius of the light
    pub fn random_in_light(&self) -> Vector {
        let mut rng = rand::thread_rng();

        // generate random points in the cube containing sphere until one is in the sphere
        // unwrap is valid here because it is mathematically sound - at some point, it will generate a point in the sphere
        // and since this is an infinite iterator, it will eventually find such a point - find will never be None
        (0..)
            .map(|_| {
                // generate random in cube, and then offset to the centre of sphere
                Vector::new(
                    self.random.sample(&mut rng),
                    self.random.sample(&mut rng),
                    self.random.sample(&mut rng),
                ) + self.centre
            })
            .find(|&point| self.point_in_light(point))
            .unwrap()
    }
}
