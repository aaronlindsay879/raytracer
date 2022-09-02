use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    /// Gets the inner colour values as a float array
    pub fn to_inner(self) -> [f64; 3] {
        [self.r, self.g, self.b]
    }

    /// Clamp each colour value to [0, 1]
    pub fn clamp(&self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Colour) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl From<[f64; 3]> for Colour {
    fn from(val: [f64; 3]) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
        }
    }
}
