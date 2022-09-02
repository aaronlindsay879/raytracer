#![feature(is_some_with)]

mod colour;
mod light;
mod material;
mod ray;
mod scene;
mod sphere;
mod tracer;
mod vector;

use colour::Colour;
use image::RgbImage;
use indicatif::ParallelProgressIterator;
use light::Light;
use material::Material;
use rayon::prelude::*;
use scene::Scene;
use sphere::Sphere;
use tracer::Tracer;
use vector::Vector;

const HEIGHT: u32 = 1920;
const WIDTH: u32 = 2560;

fn main() {
    let mut scene = Scene {
        ambient_light: Colour::new(0.5, 0.5, 0.5),
        ..Default::default()
    };

    scene.add_sphere(Sphere::new(
        Vector::new(0.0, 0.0, 2.0),
        1.3,
        Material::new([0.2, 0.1, 0.1], [0.4, 0.1, 0.1], [0.7, 0.7, 0.7], 100.0),
    ));
    scene.add_sphere(Sphere::new(
        Vector::new(0.2, 0.0, 0.2),
        0.1,
        Material::new([0.1, 0.2, 0.1], [0.5, 0.9, 0.5], [0.7, 0.7, 0.7], 25.0),
    ));
    scene.add_sphere(Sphere::new(
        Vector::new(-0.6, 0.4, 0.3),
        0.2,
        Material::new([0.1, 0.1, 0.2], [0.5, 0.5, 0.9], [0.7, 0.7, 0.7], 50.0),
    ));
    scene.add_sphere(Sphere::new(
        Vector::new(0.6, -1.7, 3.0),
        0.6,
        Material::new([0.1, 0.1, 0.2], [0.5, 0.5, 0.9], [0.7, 0.7, 0.7], 50.0),
    ));

    scene.add_light(Light::new(
        Vector::new(0.3, 0.3, -0.5),
        Colour::new(0.8, 0.8, 0.8),
        Colour::new(0.8, 0.8, 0.8),
    ));
    // scene.add_light(Light::new(
    //     Vector::new(5.0, -1.0, 0.5),
    //     Colour::new(0.7, 0.7, 0.7),
    //     Colour::new(0.8, 0.8, 0.8),
    // ));

    let tracer = Tracer::new(scene, WIDTH, HEIGHT);
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    image
        .enumerate_pixels_mut()
        .par_bridge()
        .progress_count(WIDTH as u64 * HEIGHT as u64)
        .for_each(|(x, y, pixel)| {
            *pixel = tracer.colour_at_pixel(x, y);
        });

    image.save("out.png").unwrap();
}
