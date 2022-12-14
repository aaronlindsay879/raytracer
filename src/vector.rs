use forward_ref_generic::{commutative_binop, forward_ref_commutative_binop};
use serde_tuple::Deserialize_tuple;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Default, Deserialize_tuple)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    /// Constructs a vector from the given values.
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Linearly interpolates between two vectors.
    /// E.g. (0, 1, 2) and (2, 1, 0) with an amount of 0.5 would give (1, 1, 1)
    pub fn lerp(&self, other: &Vector, amount: f64) -> Self {
        (1.0 - amount) * self + amount * other
    }

    /// Calculates the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalises a vector such that it points in same direction as before but has magnitude of 1
    pub fn normalise(&self) -> Self {
        *self / self.magnitude()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

commutative_binop! {
    impl Mul for Vector, f64
}

forward_ref_commutative_binop! {
    impl Mul for Vector, f64
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul for Vector {
    type Output = f64;

    /// Dot product
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
