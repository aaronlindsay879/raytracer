use image::Rgb;

use crate::{ray::Ray, scene::Scene};

pub struct Tracer {
    scene: Scene,
    width: u32,
    height: u32,
}

impl Tracer {
    /// Constructs a new tracer with the given scene, width and height
    pub fn new(scene: Scene, width: u32, height: u32) -> Self {
        Self {
            scene,
            width,
            height,
        }
    }

    /// Finds the colour at a given pixel by casting a ray and checking intersections with the scene
    pub fn colour_at_pixel(&self, x: u32, y: u32) -> Rgb<u8> {
        // percentage across the width and height of canvas
        let alpha = (x as f64) / (self.width as f64);
        let beta = ((self.height - y - 1) as f64) / (self.height as f64);

        // bilinear interpolation to find point at correct position
        let top = Scene::TOP_LEFT.lerp(&Scene::TOP_RIGHT, alpha);
        let bottom = Scene::BOTTOM_LEFT.lerp(&Scene::BOTTOM_RIGHT, alpha);
        let point = top.lerp(&bottom, beta);

        // construct a ray pointing towards the point from the camera, and calculate green/red values pased on ray direction
        let ray = Ray::towards_camera(point);

        // figure out if ray intersects with any spheres
        let sphere_intersect = self.scene.sphere_intersect(ray);

        // if intersect occurs, use sphere colour - otherwise black
        match sphere_intersect {
            Some(sphere) => Rgb(sphere.colour.map(|x| (x * 255.0) as u8)),
            None => Rgb([0, 0, 0]),
        }
    }
}
