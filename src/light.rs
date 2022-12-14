use crate::{colour::Colour, vector::Vector};
use rand::{distributions::Uniform, prelude::Distribution};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Light {
    pub centre: Vector,
    pub radius: f64,
    pub diffuse_intensity: Colour,
    pub specular_intensity: Colour,
}

impl Light {
    /// Simple helper function to check if a point is within the light
    fn point_in_light(&self, vector: Vector) -> bool {
        (self.centre - vector).magnitude() <= self.radius
    }

    /// Generate a random point within the radius of the light
    pub fn random_in_light(&self) -> Vector {
        let mut rng = rand::thread_rng();
        let random = Uniform::from(-self.radius..self.radius);

        // generate random points in the cube containing sphere until one is in the sphere
        // unwrap is valid here because it is mathematically sound - at some point, it will generate a point in the sphere
        // and since this is an infinite iterator, it will eventually find such a point - find will never be None
        (0..)
            .map(|_| {
                // generate random in cube, and then offset to the centre of sphere
                Vector::new(
                    random.sample(&mut rng),
                    random.sample(&mut rng),
                    random.sample(&mut rng),
                ) + self.centre
            })
            .find(|&point| self.point_in_light(point))
            .unwrap()
    }
}
