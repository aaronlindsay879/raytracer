use crate::{colour::Colour, light::Light, ray::Ray, shapes::Shape, vector::Vector};
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
#[serde(default)]
pub struct Scene {
    #[serde(alias = "sphere")]
    pub shapes: Vec<Shape>,
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
    pub antialiasing: usize,
}

impl Scene {
    /// Returns the first shape the ray intersects with, if any
    pub fn shape_intersect(&self, ray: Ray) -> Option<(f64, &Shape)> {
        // find the closest shape that intersects, by first figuring out all the shape that intersect with the ray
        // and then choosing the closest one
        self.shapes
            .iter()
            .filter_map(|shape| shape.ray_intersect(&ray).map(|val| (val, shape)))
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    }
}
