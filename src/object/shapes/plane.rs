use std::f64::{INFINITY, NEG_INFINITY};

use derive_new::new;

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{float::approx_eq, Point, Ray, Vector},
    Shape,
};

/// A `Plane` is an infinitely large plane situated along the x and z axes.
#[derive(Clone, Copy, Debug, new)]
pub struct Plane;

impl Intersectable for Plane {
    #[must_use]
    fn intersect<'a>(&self, ray: &Ray, object: &'a Shape) -> Option<List<'a>> {
        if approx_eq!(ray.direction.y, 0.0) {
            return None;
        }

        Some(List::from(Intersection::new(
            object,
            -ray.origin.y / ray.direction.y,
        )))
    }

    #[must_use]
    fn normal_at(
        &self,
        _point: &Point,
        _intersection: &Intersection,
    ) -> Vector {
        Vector::y_axis()
    }
}

impl Bounded for Plane {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point::new(NEG_INFINITY, 0.0, NEG_INFINITY),
            Point::new(INFINITY, 0.0, INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, object::shapes::Shapes, Object};

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let o = Object::plane_builder().build();

        let Shapes::Plane(p) = &o.shape else { unreachable!() };

        assert!(p
            .intersect(
                &Ray::new(Point::new(0.0, 10.0, 0.0), Vector::z_axis()),
                &o
            )
            .is_none());

        assert!(p
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()), &o)
            .is_none());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let o = Object::plane_builder().build();

        let Shapes::Plane(p) = &o.shape else { unreachable!() };

        let l = p
            .intersect(
                &Ray::new(Point::new(0.0, 1.0, 0.0), -Vector::y_axis()),
                &o,
            )
            .unwrap();

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let o = Object::plane_builder().build();

        let Shapes::Plane(p) = &o.shape else { unreachable!() };

        let l = p
            .intersect(
                &Ray::new(Point::new(0.0, -1.0, 0.0), Vector::y_axis()),
                &o,
            )
            .unwrap();

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].t, 1.0);
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let o = Object::plane_builder().build();

        let Shapes::Plane(p) = &o.shape else { unreachable!() };

        let i = Intersection::new(&o, 0.0);

        let n = Vector::y_axis();

        assert_approx_eq!(p.normal_at(&Point::origin(), &i), n);
        assert_approx_eq!(p.normal_at(&Point::new(10.0, 0.0, -10.0), &i), n);
        assert_approx_eq!(p.normal_at(&Point::new(-5.0, 0.0, 150.0), &i), n);
    }

    #[test]
    fn the_bounding_box_of_a_plane() {
        let p = Plane::new();

        assert_approx_eq!(
            p.bounding_box(),
            BoundingBox::new(
                Point::new(NEG_INFINITY, 0.0, NEG_INFINITY),
                Point::new(INFINITY, 0.0, INFINITY)
            )
        );
    }
}
