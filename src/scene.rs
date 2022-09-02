use crate::{sphere::Sphere, vector::Vector};

#[derive(Default)]
pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    /// Top left of canvas.
    pub const TOP_LEFT: Vector = Vector::new(1.0, 0.75, 0.0);
    /// Top right of canvas.
    pub const TOP_RIGHT: Vector = Vector::new(-1.0, 0.75, 0.0);
    /// Bottom left of canvas.
    pub const BOTTOM_LEFT: Vector = Vector::new(1.0, -0.75, 0.0);
    /// Bottom right of canvas.
    pub const BOTTOM_RIGHT: Vector = Vector::new(-1.0, -0.75, 0.0);

    /// Camera position.
    pub const CAMERA: Vector = Vector::new(0.0, 0.0, -1.0);

    /// Adds a sphere to the scene
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    /// Returns reference to the current spheres in the scene
    pub fn spheres(&self) -> &[Sphere] {
        &self.spheres
    }
}
