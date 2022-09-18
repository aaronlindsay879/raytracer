use image::Rgb;

use crate::{colour::Colour, ray::Ray, scene::Scene};

pub struct Tracer<'a> {
    scene: &'a Scene,
    width: u32,
    height: u32,
}

impl<'a> Tracer<'a> {
    /// Constructs a new tracer with the given scene, width and height
    pub fn new(scene: &'a Scene, width: u32, height: u32) -> Self {
        Self {
            scene,
            width,
            height,
        }
    }

    fn recurse_colour(&self, ray: &Ray, recurses: usize) -> Colour {
        // if no shape intersection, return background colour (black)
        if let Some((t, shape)) = self.scene.shape_intersect(*ray) {
            // otherwise calculate intersection point and the view vector
            let intersect_point = ray.origin + t * ray.direction;
            let view = (ray.direction * -1.0).normalise();

            // then calculate the surface normal
            let normal = shape.normal(&intersect_point);

            // and then find the colour at that point
            let colour = shape.lighting(self.scene, intersect_point, view, normal);

            // if we haven't yet hit the recursion limit, calculate the reflected vector and calculate again
            // adding the colour to the currently stored colour
            if recurses < self.scene.recursion_depth_limit {
                let reflectance_vector = normal * (normal * view) * 2.0 - view;

                let reflected_colour = self
                    .recurse_colour(&Ray::new(intersect_point, reflectance_vector), recurses + 1);

                colour + (reflected_colour * shape.material().reflectiveness)
            } else {
                colour
            }
        } else {
            Colour::new(0.0, 0.0, 0.0)
        }
    }

    /// Finds the colour at a given pixel by casting a ray and checking intersections with the scene
    pub fn colour_at_pixel(&self, x: u32, y: u32) -> Rgb<u8> {
        // variables for antialiasing - how to lay out subpixel grid
        let offset = 1.0 / (self.scene.antialiasing as f64).powi(2);
        let step_offset = 1.0 / (self.scene.antialiasing as f64);
        let div_factor = self.scene.antialiasing as f64 * self.scene.antialiasing as f64;

        // sum up all colours from subpixel grid and divide to find an average
        let colour: Colour = (1..=self.scene.antialiasing)
            .map(|x_step| {
                (1..=self.scene.antialiasing)
                    .map(|y_step| {
                        // percentage across the width and height of canvas
                        let alpha = ((x as f64 - offset) - step_offset * x_step as f64)
                            / (self.width as f64);
                        let beta = ((y as f64 - offset) - step_offset * y_step as f64)
                            / (self.height as f64);

                        // bilinear interpolation to find point at correct position
                        let top = self.scene.top_left.lerp(&self.scene.top_right, alpha);
                        let bottom = self.scene.bottom_left.lerp(&self.scene.bottom_right, alpha);
                        let point = top.lerp(&bottom, beta);

                        // construct a ray pointing towards the point from the camera, and calculate green/red values pased on ray direction
                        let ray = Ray::towards_camera(point, self.scene.camera);

                        // if intersect occurs, use calculated shape colour - otherwise black
                        self.recurse_colour(&ray, 0)
                    })
                    .fold(Colour::new(0.0, 0.0, 0.0), |a, b| a + b)
            })
            .fold(Colour::new(0.0, 0.0, 0.0), |a, b| a + b)
            / div_factor;

        // then clamp colours to [0, 1] and convert to u8
        Rgb(colour.clamp().to_inner().map(|x| (x * 255.0) as u8))
    }
}
