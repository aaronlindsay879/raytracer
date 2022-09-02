use crate::vector::Vector;

#[derive(Debug)]
pub struct Sphere {
    pub point: Vector,
    pub radius: f64,
    pub colour: [f64; 3],
}

impl Sphere {
    pub fn new(point: Vector, radius: f64, colour: [f64; 3]) -> Self {
        Self {
            point,
            radius,
            colour,
        }
    }
}
