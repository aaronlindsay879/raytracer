mod ray;
mod scene;
mod sphere;
mod tracer;
mod vector;

use image::RgbImage;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use scene::Scene;
use sphere::Sphere;
use tracer::Tracer;
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
