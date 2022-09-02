use image::Rgb;

use crate::{colour::Colour, material::Material, ray::Ray, scene::Scene, vector::Vector};

#[derive(Debug)]
pub struct Sphere {
    pub point: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(point: Vector, radius: f64, material: Material) -> Self {
        Self {
            point,
            radius,
            material,
        }
    }

    /// Returns the minimum distance (if applicable) for ray-sphere intersection
    pub fn ray_intersect(&self, ray: &Ray) -> Option<f64> {
        // perform sphere intersection test using quadratic eqn
        let a = ray.direction.magnitude().powi(2);
        let b = 2.0 * ((ray.origin - self.point) * ray.direction);
        let c = (ray.origin - self.point).magnitude().powi(2) - self.radius.powi(2);

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
        let surface_normal = (intersect_point - self.point).normalise();

        // start off with ambient colour
        let mut colour = self.material.ambient_constant * scene.ambient_light;

        // then add all the diffuse and specular terms from every light
        colour += scene
            .lights
            .iter()
            .filter_map(|light| {
                let light_vector = (light.point - intersect_point).normalise();

                // ignore lights that face the inside of the sphere
                let direction = surface_normal * light_vector;
                if direction <= 0.0 {
                    None
                } else {
                    // calculate diffuse term
                    let diffuse =
                        self.material.diffuse_constant * light.diffuse_intensity * direction;

                    // calculate specular term
                    let reflectance = (2.0 * direction * surface_normal - light_vector).normalise();
                    let view = (Scene::CAMERA - intersect_point).normalise();

                    let specular = self.material.specular_constant
                        * light.specular_intensity
                        * (reflectance * view).powf(self.material.shininess);

                    // total lighting for light is sum
                    Some(diffuse + specular)
                }
            })
            .fold(Colour::new(0.0, 0.0, 0.0), |a, b| a + b);

        // then clamp colours to [0, 1] and convert to u8
        Rgb(colour.clamp().to_inner().map(|x| (x * 255.0) as u8))
    }
}
