use serde::Deserialize;

use crate::{material::Material, ray::Ray, vector::Vector};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Sphere {
    pub centre: Vector,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
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
}
