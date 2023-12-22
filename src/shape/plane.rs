use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{float::approx_eq, Point, Ray, Vector},
};

/// A `Plane` is an infinitely large plane situated along the x and z axes.
#[derive(Clone, Copy, Debug)]
pub struct Plane;

impl Intersectable for Plane {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        if approx_eq!(ray.direction.y, 0.0) {
            return None;
        }

        todo!()
    }

    fn normal_at(&self, _point: &Point) -> Vector {
        Vector::y_axis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane;

        assert!(p
            .intersect(&Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_axis()))
            .is_none());

        assert!(p
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane;

        let n = Vector::y_axis();

        assert_approx_eq!(p.normal_at(&Point::origin()), n);
        assert_approx_eq!(p.normal_at(&Point::new(10.0, 0.0, -10.0)), n);
        assert_approx_eq!(p.normal_at(&Point::new(-5.0, 0.0, 150.0)), n);
    }
}
