use float_cmp::{ApproxEq, F64Margin};

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
    fn comparing_vectors() {
        let v1 = Vector::new(0.0, -1.0, 2.5);
        let v2 = Vector::new(-0.0, -1.0, 2.5);
        let v3 = Vector::new(0.000_06, -1.0, 2.5);

        assert_approx_eq!(v1, v2);

        assert_approx_ne!(v1, v3);
    }
}
