use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// Constructs a ray to the global camera from a given point
    pub fn towards_camera(point: Vector, camera: Vector) -> Self {
        Self {
            origin: point,
            direction: point - camera,
        }
    }
}
