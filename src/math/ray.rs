use derive_more::Constructor;

use super::{
    float::impl_approx_eq, Point, Transformable, Transformation, Vector,
};

/// A Ray represents a geometric vector with a specific origin point and
/// pointing in some direction.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    #[must_use]
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl<'a> Transformable<'a> for Ray {
    fn apply(&'a self, transformation: &Transformation) -> Self {
        Self::new(
            self.origin.apply(transformation),
            self.direction.apply(transformation),
        )
    }
}

impl_approx_eq!(Ray { origin, direction });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_ray() {
        let p = Point::new(1.0, 2.0, 3.0);
        let v = Vector::new(4.0, 5.0, 6.0);

        let r = Ray::new(p, v);

        assert_approx_eq!(r.origin, p);
        assert_approx_eq!(r.direction, v);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::x_axis());

        assert_approx_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_approx_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_approx_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_approx_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        assert_approx_eq!(
            Ray::new(Point::new(1.0, 2.0, 3.0), Vector::y_axis())
                .apply(&Transformation::new().translate(3.0, 4.0, 5.0)),
            Ray::new(Point::new(4.0, 6.0, 8.0), Vector::y_axis())
        );
    }

    #[test]
    fn scaling_a_ray() {
        assert_approx_eq!(
            Ray::new(Point::new(1.0, 2.0, 3.0), Vector::y_axis())
                .apply(&Transformation::new().scale(2.0, 3.0, 4.0)),
            Ray::new(Point::new(2.0, 6.0, 12.0), Vector::new(0.0, 3.0, 0.0))
        );
    }

    #[test]
    fn comparing_rays() {
        let r1 =
            Ray::new(Point::new(1.06, 0.0, -20.5), Vector::new(1.0, 0.0, -2.7));
        let r2 =
            Ray::new(Point::new(1.06, 0.0, -20.5), Vector::new(1.0, 0.0, -2.7));
        let r3 = Ray::new(
            Point::new(1.06, 0.000_09, -20.5),
            Vector::new(1.0, 0.0, -2.7),
        );

        assert_approx_eq!(r1, r2);

        assert_approx_ne!(r1, r3);
    }
}
