//! A `Plane` is an infinitely large plane situated along the x and z axes.

use crate::{
    intersection::ListBuilder,
    math::{float::approx_eq, Point, Ray, Vector},
};

#[must_use]
pub fn intersect<'a>(ray: &Ray) -> Option<ListBuilder<'a>> {
    if approx_eq!(ray.direction.y, 0.0) {
        return None;
    }

    Some(ListBuilder::new().add_t(-ray.origin.y / ray.direction.y))
}

#[must_use]
pub fn normal_at(_point: &Point) -> Vector {
    Vector::y_axis()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Object};

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        assert!(intersect(&Ray::new(
            Point::new(0.0, 10.0, 0.0),
            Vector::z_axis()
        ))
        .is_none());

        assert!(
            intersect(&Ray::new(Point::origin(), Vector::z_axis())).is_none()
        );
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let b =
            intersect(&Ray::new(Point::new(0.0, 1.0, 0.0), -Vector::y_axis()));

        assert!(b.is_some());

        let o = Object::default_plane();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 1);
        assert_approx_eq!(i[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let b =
            intersect(&Ray::new(Point::new(0.0, -1.0, 0.0), Vector::y_axis()));

        assert!(b.is_some());

        let o = Object::default_plane();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 1);
        assert_approx_eq!(i[0].t, 1.0);
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let n = Vector::y_axis();

        assert_approx_eq!(normal_at(&Point::origin()), n);
        assert_approx_eq!(normal_at(&Point::new(10.0, 0.0, -10.0)), n);
        assert_approx_eq!(normal_at(&Point::new(-5.0, 0.0, 150.0)), n);
    }
}
