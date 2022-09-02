mod ray;
mod scene;
mod sphere;
mod vector;

use image::{Rgb, RgbImage};
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use vector::Vector;

const HEIGHT: u32 = 1920;
const WIDTH: u32 = 2560;

fn main() {
    let mut scene = Scene::default();
    scene.add_sphere(Sphere::new(
        Vector::new(0.0, 0.0, 2.0),
        1.3,
        [0.5, 0.5, 0.25],
    ));
    scene.add_sphere(Sphere::new(
        Vector::new(0.2, 0.0, 0.2),
        0.1,
        [0.25, 0.5, 0.5],
    ));
    scene.add_sphere(Sphere::new(
        Vector::new(-0.6, 0.4, 0.3),
        0.2,
        [0.5, 0.25, 0.5],
    ));

    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // percentage across the width and height of canvas
            let alpha = (x as f64) / (WIDTH as f64);
            let beta = ((HEIGHT - y - 1) as f64) / (HEIGHT as f64);

            // bilinear interpolation to find point at correct position
            let top = Scene::TOP_LEFT.lerp(&Scene::TOP_RIGHT, alpha);
            let bottom = Scene::BOTTOM_LEFT.lerp(&Scene::BOTTOM_RIGHT, alpha);
            let point = top.lerp(&bottom, beta);

            // construct a ray pointing towards the point from the camera, and calculate green/red values pased on ray direction
            let ray = Ray::towards_camera(point);

            // figure out if ray intersects with any spheres
            let sphere_intersect = scene.sphere_intersect(ray);

            // if intersect occurs, use sphere colour - otherwise black
            let colour = match sphere_intersect {
                Some(sphere) => Rgb(sphere.colour.map(|x| (x * 255.0) as u8)),
                None => Rgb([0, 0, 0]),
            };

            image.put_pixel(x, y, colour);
        }
    }

    image.save("out.png").unwrap();
}
