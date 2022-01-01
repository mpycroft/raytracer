use crate::{
    intersect::IntersectionPoints,
    math::{approx::FLOAT_EPSILON, Point, Ray, Vector},
    Intersectable,
};

/// A Plane is an infinitely large plane situated along the x and z axes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Plane;

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionPoints> {
        if ray.direction.y.abs() < FLOAT_EPSILON {
            return None;
        }

        Some(vec![-ray.origin.y / ray.direction.y].into())
    }

    fn normal_at(&self, _point: &Point) -> Vector {
        Vector::y_axis()
    }
}

add_approx_traits!(Plane { true });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn intersect() {
        let p = Plane;

        let i = p
            .intersect(&Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_axis()));

        assert!(i.is_none());

        let i =
            p.intersect(&Ray::new(Point::new(0.0, 0.0, 0.0), Vector::z_axis()));

        assert!(i.is_none());

        let i = p
            .intersect(&Ray::new(Point::new(0.0, 1.0, 0.0), -Vector::y_axis()))
            .unwrap();

        assert_eq!(i.len(), 1);
        assert_float_relative_eq!(i[0], 1.0);

        let i = p
            .intersect(&Ray::new(Point::new(0.0, -1.0, 0.0), Vector::y_axis()))
            .unwrap();

        assert_eq!(i.len(), 1);
        assert_float_relative_eq!(i[0], 1.0);
    }

    #[test]
    fn normal_at() {
        let p = Plane;

        assert_relative_eq!(p.normal_at(&Point::origin()), Vector::y_axis());
        assert_relative_eq!(
            p.normal_at(&Point::new(10.0, 0.0, -10.0)),
            Vector::y_axis()
        );
        assert_relative_eq!(
            p.normal_at(&Point::new(-5.0, 0.0, 150.0)),
            Vector::y_axis()
        );
    }

    #[test]
    fn approx() {
        let p1 = Plane;
        let p2 = Plane;

        assert_abs_diff_eq!(p1, p2);

        assert_relative_eq!(p1, p2);

        assert_ulps_eq!(p1, p2);
    }
}
