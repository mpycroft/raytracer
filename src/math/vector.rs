use float_cmp::{ApproxEq, F64Margin};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A Vector is a representation of a geometric vector, pointing in a given
/// direction and with a magnitude.
#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ApproxEq for Vector {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.x.approx_eq(other.x, margin)
            && self.y.approx_eq(other.y, margin)
            && self.z.approx_eq(other.z, margin)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::float::{assert_approx_eq, assert_approx_ne};

    use super::*;

    #[test]
    fn creating_a_vector() {
        let p = Vector::new(2.8, 4.0, -0.7);

        assert_approx_eq!(p.x, 2.8);
        assert_approx_eq!(p.y, 4.0);
        assert_approx_eq!(p.z, -0.7);
    }

    #[test]
    fn adding_two_vectors() {
        assert_approx_eq!(
            Vector::new(2.3, 5.1, -3.0) + Vector::new(1.0, 1.0, 1.0),
            Vector::new(3.3, 6.1, -2.0)
        );

        let mut v = Vector::new(-0.6, 0.5, 1.2);
        v += Vector::new(-0.0, 0.5, -0.2);

        assert_approx_eq!(v, Vector::new(-0.6, 1.0, 1.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        assert_approx_eq!(
            Vector::new(1.0, 2.0, 3.0) - Vector::new(3.0, 2.0, 1.0),
            Vector::new(-2.0, 0.0, 2.0)
        );

        let mut v = Vector::new(-1.0, -2.0, -3.0);
        v -= Vector::new(-2.0, 2.5, -0.1);

        assert_approx_eq!(v, Vector::new(1.0, -4.5, -2.9));
    }

    #[test]
    fn comparing_vectors() {
        let v1 = Vector::new(0.0, -1.0, 2.5);
        let v2 = Vector::new(-0.0, -1.0, 2.5);
        let v3 = Vector::new(0.000_06, -1.0, 2.5);

        assert_approx_eq!(v1, v2);

        assert_approx_ne!(v1, v3);
    }
}
