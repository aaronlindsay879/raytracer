use crate::{scene::Scene, vector::Vector};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    /// Constructs a ray to the global camera from a given point
    pub fn towards_camera(point: Vector) -> Self {
        Self {
            origin: point,
            direction: point - Scene::CAMERA,
        }
    }
}
