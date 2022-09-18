use std::sync::RwLock;

use serde::Deserialize;

use crate::{material::Material, ray::Ray, vector::Vector};

#[derive(Debug, Deserialize)]

pub struct Plane {
    pub points: [Vector; 3],
    pub material: Material,

    #[serde(skip)]
    normal: RwLock<Option<Vector>>,
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.material == other.material
    }
}

impl Plane {
    pub fn normal(&self, pairs: [(usize, usize); 2]) -> Vector {
        let data = self.normal.read().unwrap();
        match *data {
            Some(normal) => normal,
            None => {
                drop(data);

                let a = self.points[pairs[0].0] - self.points[pairs[0].1];
                let b = self.points[pairs[1].0] - self.points[pairs[1].1];

                let normal = Vector {
                    x: a.z * b.y - a.y * b.z,
                    y: a.x * b.z - a.z * b.x,
                    z: a.y * b.x - a.x * b.y,
                };

                *self.normal.write().unwrap() = Some(normal);
                normal
            }
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal([(0, 1), (1, 2)]).normalise();

        let bottom =
            normal.x * ray.direction.x + normal.y * ray.direction.y + normal.z * ray.direction.z;

        if bottom == 0.0 {
            None
        } else {
            let result = (normal.x * (self.points[0].x - ray.origin.x)
                + normal.y * (self.points[0].y - ray.origin.y)
                + normal.z * (self.points[0].z - ray.origin.z))
                / bottom;

            if result.is_sign_positive() {
                Some(result)
            } else {
                None
            }
        }
    }
}
