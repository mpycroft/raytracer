use super::{Matrix, Point, Vector};

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

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, transform: &Matrix<4>) -> Self {
        Self::new(*transform * self.origin, *transform * self.direction)
    }
}

add_approx_traits!(Ray { origin, direction });

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

    #[test]
    fn position() {
        let p = Point::new(2.0, 3.0, 4.0);
        let r = Ray::new(p, Vector::new(1.0, 0.0, 0.0));

        assert_relative_eq!(r.position(0.0), p);
        assert_relative_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_relative_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_relative_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn transform() {
        let v = Vector::new(0.0, 1.0, 0.0);
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), v);

        assert_relative_eq!(
            r.transform(&Matrix::translate(3.0, 4.0, 5.0)),
            Ray::new(Point::new(4.0, 6.0, 8.0), v)
        );

        assert_relative_eq!(
            r.transform(&Matrix::scale(2.0, 3.0, 4.0)),
            Ray::new(Point::new(2.0, 6.0, 12.0), Vector::new(0.0, 3.0, 0.0))
        );
    }

    #[test]
    fn approx() {
        let r1 =
            Ray::new(Point::new(0.0, 1.5, -2.3), Vector::new(9.5, 0.1, 0.5));
        let r2 =
            Ray::new(Point::new(0.0, 1.5, -2.3), Vector::new(9.5, 0.1, 0.5));
        let r3 = Ray::new(
            Point::new(0.000_01, 1.5, -2.3),
            Vector::new(9.502, 0.1, 0.5),
        );

        assert_abs_diff_eq!(r1, r2);
        assert_abs_diff_ne!(r1, r3);

        assert_relative_eq!(r1, r2);
        assert_relative_ne!(r1, r3);

        assert_ulps_eq!(r1, r2);
        assert_ulps_ne!(r1, r3);
    }
}
