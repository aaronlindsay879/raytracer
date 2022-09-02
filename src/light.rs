use crate::{colour::Colour, vector::Vector};

pub struct Light {
    pub point: Vector,
    pub diffuse_intensity: Colour,
    pub specular_intensity: Colour,
}

impl Light {
    pub fn new(point: Vector, diffuse_intensity: Colour, specular_intensity: Colour) -> Self {
        Self {
            point,
            diffuse_intensity,
            specular_intensity,
        }
    }
}
