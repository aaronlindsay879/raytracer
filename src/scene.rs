use crate::{colour::Colour, light::Light, ray::Ray, sphere::Sphere, vector::Vector};
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
#[serde(default)]
pub struct Scene {
    #[serde(alias = "sphere")]
    pub spheres: Vec<Sphere>,
    #[serde(alias = "light")]
    pub lights: Vec<Light>,
    pub ambient_light: Colour,

    pub top_left: Vector,
    pub top_right: Vector,
    pub bottom_left: Vector,
    pub bottom_right: Vector,
    pub camera: Vector,

    pub num_light_points: usize,
    pub width: u32,
    pub height: u32,

    pub recursion_depth_limit: usize,
}

impl Scene {
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
