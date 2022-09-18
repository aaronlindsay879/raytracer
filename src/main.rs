#![feature(is_some_with, once_cell)]

mod colour;
mod light;
mod material;
mod ray;
mod scene;
mod shapes;
mod tracer;
mod vector;

use image::RgbImage;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use scene::Scene;
use std::error::Error;
use tracer::Tracer;

fn main() -> Result<(), Box<dyn Error>> {
    // parse options from file
    let options = std::fs::read_to_string("options.toml")?;
    let scene: Scene = toml::from_str(&options)?;

    println!("{:#?}", scene);

    let tracer = Tracer::new(&scene, scene.width, scene.height);
    let mut image = RgbImage::new(scene.width, scene.height);

    image
        .enumerate_pixels_mut()
        .par_bridge()
        .progress_count(scene.width as u64 * scene.height as u64)
        .for_each(|(x, y, pixel)| {
            *pixel = tracer.colour_at_pixel(x, y);
        });

    image.save("out.png")?;

    Ok(())
}
