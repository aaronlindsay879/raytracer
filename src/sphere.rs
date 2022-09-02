use image::Rgb;
use serde::Deserialize;

use crate::{colour::Colour, material::Material, ray::Ray, scene::Scene, vector::Vector};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    /// Returns the minimum distance (if applicable) for ray-sphere intersection
    pub fn ray_intersect(&self, ray: &Ray) -> Option<f64> {
        // perform sphere intersection test using quadratic eqn
        let a = ray.direction.magnitude().powi(2);
        let b = 2.0 * ((ray.origin - self.centre) * ray.direction);
        let c = (ray.origin - self.centre).magnitude().powi(2) - self.radius.powi(2);

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
                Some(t_1.min(t_2))
            } else {
                // if only one positive, choose the max (only positive solution) so it occurs after image plane
                Some(t_1.max(t_2))
            }
        }
    }

    /// Calculates the colour of the sphere from a given ray and scene.
    pub fn lighting(&self, ray: &Ray, scene: &Scene, t: f64) -> Rgb<u8> {
        // find the ray-sphere intersection point and the sphere's surface normal
        let intersect_point = ray.origin + t * ray.direction;
        let surface_normal = (intersect_point - self.centre).normalise();

        // start off with ambient colour
        let mut colour = self.material.ambient_constant * scene.ambient_light;

        // then add all the diffuse and specular terms from every light
        colour += scene
            .lights
            .iter()
            .filter_map(|light| {
                let light_vector = (light.centre - intersect_point).normalise();

                // number of rays cast from random points in the light which are blocked (shadowed)
                let num_shadowed: usize = scene
                    .spheres
                    .iter()
                    .filter(|&sphere| sphere != self)
                    .map(|sphere| {
                        (0..scene.num_light_points)
                            .filter(|_| {
                                let shadow_vector = light.random_in_light() - intersect_point;
                                let shadow_ray = Ray::new(intersect_point, shadow_vector);

                                // check if any intersects occur along the shadow ray
                                sphere
                                    .ray_intersect(&shadow_ray)
                                    .is_some_and(|&intersect| intersect > 0.0 && intersect < 1.0)
                            })
                            .count()
                    })
                    .sum();

                // ignore lights that face the inside of the sphere or that are blocked
                let direction = surface_normal * light_vector;
                if direction <= 0.0 {
                    None
                } else {
                    // calculate diffuse term
                    let diffuse =
                        self.material.diffuse_constant * light.diffuse_intensity * direction;

                    // calculate specular term
                    let reflectance = (2.0 * direction * surface_normal - light_vector).normalise();
                    let view = (scene.camera - intersect_point).normalise();

                    let specular = self.material.specular_constant
                        * light.specular_intensity
                        * (reflectance * view).powf(self.material.shininess);

                    // how much to scale the light depending on the number of blocked shadow rays
                    let scale_factor =
                        1.0 - (num_shadowed as f64) / (scene.num_light_points as f64);

                    // total lighting for light is sum
                    Some((diffuse + specular) * scale_factor)
                }
            })
            .fold(Colour::new(0.0, 0.0, 0.0), |a, b| a + b);

        // then clamp colours to [0, 1] and convert to u8
        Rgb(colour.clamp().to_inner().map(|x| (x * 255.0) as u8))
    }
}
