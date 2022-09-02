use crate::{colour::Colour, light::Light, ray::Ray, sphere::Sphere, vector::Vector};

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub ambient_light: Colour,
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

    /// Adds a light to the scene
    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    /// Returns the first sphere the ray intersects with, if any
    pub fn sphere_intersect(&self, ray: Ray) -> Option<(f64, &Sphere)> {
        // find the closest sphere that intersects, by first figuring out all the spheres that intersect with the ray
        // and then choosing the closest one
        self.spheres
            .iter()
            .filter_map(|sphere| sphere.ray_intersect(&ray).map(|val| (val, sphere)))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
