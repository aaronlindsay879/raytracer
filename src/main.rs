use image::{Rgb, RgbImage};
use ray::Ray;

mod ray;
mod scene;
mod vector;

const HEIGHT: u32 = 192;
const WIDTH: u32 = 256;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // percentage across the width and height of canvas
            let alpha = (x as f64) / (WIDTH as f64);
            let beta = ((HEIGHT - y - 1) as f64) / (HEIGHT as f64);

            // bilinear interpolation to find point at correct position
            let top = scene::TOP_LEFT.lerp(&scene::TOP_RIGHT, alpha);
            let bottom = scene::BOTTOM_LEFT.lerp(&scene::BOTTOM_RIGHT, alpha);
            let point = top.lerp(&bottom, beta);

            // construct a ray pointing towards the point from the camera, and calculate green/red values pased on ray direction
            let ray = Ray::from_camera(point);

            let red = ((ray.direction.x + 1.0) * 255.0 / 2.0) as u8;
            let green = ((ray.direction.y + 0.75) * 255.0 / 1.5) as u8;

            // write pixel with a fixed blue value
            image.put_pixel(x, y, Rgb([red, green, 108]));
        }
    }

    image.save("out.png").unwrap();
}
