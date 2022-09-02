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

            // find the closest sphere that intersects, by first figuring out all the spheres that intersect with the ray
            // and then choosing the closest one
            let sphere_intersect = scene
                .spheres()
                .iter()
                .filter_map(|sphere| {
                    // perform sphere intersection test using quadratic eqn
                    let a = ray.direction.norm().powi(2);
                    let b = 2.0 * ((ray.origin - sphere.point) * ray.direction);
                    let c = (ray.origin - sphere.point).norm().powi(2) - sphere.radius.powi(2);

                    // if discriminant < 0, then no solutions - ray did not intersect sphere
                    // if discriminant >= 0, then 1 or 2 solutions - ray either touched or intersected sphere
                    let discriminant = b.powi(2) - 4.0 * a * c;

                    if discriminant < 0.0 {
                        None
                    } else {
                        // find the two solutions to the eqn in the case that ray touches/intersects sphere
                        let t_1 = (-b + discriminant.sqrt()) / (2.0 * a);
                        let t_2 = (-b - discriminant.sqrt()) / (2.0 * a);

                        if t_1 < 0.0 && t_2 < 0.0 {
                            // if both solutions negative, then intersection occurs entirely before image plane
                            None
                        } else if t_1 >= 0.0 && t_2 >= 0.0 {
                            // if both positive, both intersections occur after image plane - so choose the closest
                            Some((t_1.min(t_2), sphere))
                        } else {
                            // if only one positive, choose the max (only positive solution) so it occurs after image plane
                            Some((t_1.max(t_2), sphere))
                        }
                    }
                })
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            // if intersect occurs, use sphere colour - otherwise black
            let colour = match sphere_intersect {
                Some((_, sphere)) => Rgb(sphere.colour.map(|x| (x * 255.0) as u8)),
                None => Rgb([0, 0, 0]),
            };

            image.put_pixel(x, y, colour);
        }
    }

    image.save("out.png").unwrap();
}
