use crate::{scene, vector::Vector};

pub struct Ray {
    origin: Vector,
    pub direction: Vector,
}

impl Ray {
    /// Constructs a ray from the global camera towards a given point
    pub fn from_camera(point: Vector) -> Self {
        Self {
            origin: point,
            direction: scene::CAMERA - point,
        }
    }
}
