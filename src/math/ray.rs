use super::{Point, Vector};

/// A Ray represents a geometric vector with a specific origin point and
/// pointing in some direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(4.0, 5.0, 6.0);

        let r = Ray::new(p, v);

        assert_relative_eq!(r.origin, p);
        assert_relative_eq!(r.direction, v);
    }
}
