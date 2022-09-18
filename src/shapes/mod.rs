mod plane;
mod sphere;

use crate::{colour::Colour, material::Material, ray::Ray, scene::Scene, vector::Vector};
use serde::Deserialize;
use sphere::Sphere;

use self::plane::Plane;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(sphere) => &sphere.material,
            Shape::Plane(plane) => &plane.material,
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Shape::Sphere(sphere) => sphere.ray_intersect(ray),
            Shape::Plane(plane) => plane.ray_intersect(ray),
        }
    }

    pub fn normal(&self, point: &Vector) -> Vector {
        match self {
            Shape::Sphere(sphere) => (*point - sphere.centre).normalise(),
            Shape::Plane(_plane) => Vector::new(0.0, 0.0, 1.0),
        }
    }

    /// Calculates the colour of the shape from a given ray and scene.
    pub fn lighting(&self, scene: &Scene, point: Vector, view: Vector, normal: Vector) -> Colour {
        // start off with ambient colour
        let mut colour = self.material().ambient_constant * scene.ambient_light;

        // then add all the diffuse and specular terms from every light
        colour += scene
            .lights
            .iter()
            .filter_map(|light| {
                let light_vector = (light.centre - point).normalise();

                // number of rays cast from random points in the light which are blocked (shadowed)
                let num_shadowed: usize = scene
                    .shapes
                    .iter()
                    .filter(|&shape| shape != self)
                    .map(|shape| {
                        (0..scene.num_light_points)
                            .filter(|_| {
                                let shadow_vector = light.random_in_light() - point;
                                let shadow_ray = Ray::new(point, shadow_vector);

                                // check if any intersects occur along the shadow ray
                                shape
                                    .ray_intersect(&shadow_ray)
                                    .is_some_and(|&intersect| intersect > 0.0 && intersect < 2.0)
                            })
                            .count()
                    })
                    .sum();

                // ignore lights that face the inside of the sphere or that are blocked
                let direction = normal * light_vector;
                if direction <= 0.0 {
                    None
                } else {
                    // calculate diffuse term
                    let diffuse =
                        self.material().diffuse_constant * light.diffuse_intensity * direction;

                    // calculate specular term
                    let reflectance = (2.0 * direction * normal - light_vector).normalise();

                    let specular = self.material().specular_constant
                        * light.specular_intensity
                        * (reflectance * view).powf(self.material().shininess);

                    // how much to scale the light depending on the number of blocked shadow rays
                    let scale_factor =
                        1.0 - (num_shadowed as f64) / (scene.num_light_points as f64);

                    // total lighting for light is sum
                    Some((diffuse + specular) * scale_factor)
                }
            })
            .fold(Colour::new(0.0, 0.0, 0.0), |a, b| a + b);

        colour
    }
}
