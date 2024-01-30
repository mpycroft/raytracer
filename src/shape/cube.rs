use derive_new::new;

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{Point, Ray, Vector},
    Object,
};

/// A `Cube` is an axis aligned cube of size 2 (-1.0..1.0) on each axis.
#[derive(Clone, Copy, Debug, new)]
pub struct Cube;

impl Intersectable for Cube {
    #[must_use]
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Option<List<'a>> {
        let (x_min, x_max) =
            BoundingBox::check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (y_min, y_max) =
            BoundingBox::check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (z_min, z_max) =
            BoundingBox::check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);

        let min = x_min.max(y_min).max(z_min);
        let max = x_max.min(y_max).min(z_max);

        if min > max || max < 0.0 {
            return None;
        }

        Some(List::from(vec![
            Intersection::new(object, min),
            Intersection::new(object, max),
        ]))
    }

    #[must_use]
    fn normal_at(&self, point: &Point, _intersection: &Intersection) -> Vector {
        let abs_x = point.x.abs();
        let abs_y = point.y.abs();
        let abs_z = point.z.abs();

        if abs_x >= abs_y {
            if abs_x >= abs_z {
                return Vector::new(point.x, 0.0, 0.0);
            }

            return Vector::new(0.0, 0.0, point.z);
        } else if abs_y >= abs_z {
            return Vector::new(0.0, point.y, 0.0);
        }

        Vector::new(0.0, 0.0, point.z)
    }
}

impl Bounded for Cube {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, shape::Shape};

    #[test]
    fn a_ray_intersects_a_cube() {
        let o = Object::cube_builder().build();

        let Shape::Cube(c) = &o.shape else { unreachable!() };

        let test = |r, t1, t2| {
            let l = c.intersect(&r, &o).unwrap();

            assert_approx_eq!(l[0].t, t1);
            assert_approx_eq!(l[1].t, t2);
        };

        test(Ray::new(Point::new(5.0, 0.5, 0.0), -Vector::x_axis()), 4.0, 6.0);
        test(Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::x_axis()), 4.0, 6.0);
        test(Ray::new(Point::new(0.5, 5.0, 0.0), -Vector::y_axis()), 4.0, 6.0);
        test(Ray::new(Point::new(0.5, -5.0, 0.0), Vector::y_axis()), 4.0, 6.0);
        test(Ray::new(Point::new(0.5, 0.0, 5.0), -Vector::z_axis()), 4.0, 6.0);
        test(Ray::new(Point::new(0.5, 0.0, -5.0), Vector::z_axis()), 4.0, 6.0);

        test(Ray::new(Point::new(0.0, 0.5, 0.0), Vector::z_axis()), -1.0, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let o = Object::cube_builder().build();

        let Shape::Cube(c) = &o.shape else { unreachable!() };

        let test = |r| {
            let l = c.intersect(&r, &o);

            assert!(l.is_none());
        };

        test(Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.267_3, 0.534_5, 0.801_8),
        ));
        test(Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.801_8, 0.267_3, 0.534_5),
        ));
        test(Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.534_5, 0.801_8, 0.267_3),
        ));
        test(Ray::new(Point::new(2.0, 0.0, 2.0), -Vector::z_axis()));
        test(Ray::new(Point::new(0.0, 2.0, 2.0), -Vector::y_axis()));
        test(Ray::new(Point::new(2.0, 2.0, 0.0), -Vector::x_axis()));

        test(Ray::new(Point::new(0.0, 0.0, 2.0), Vector::z_axis()));
    }

    #[test]
    fn the_normal_on_a_cube() {
        let o = Object::cube_builder().build();

        let Shape::Cube(c) = &o.shape else { unreachable!() };

        let i = Intersection::new(&o, 0.0);

        assert_approx_eq!(
            c.normal_at(&Point::new(1.0, 0.5, -0.8), &i),
            Vector::x_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-1.0, -0.2, 0.9), &i),
            -Vector::x_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-0.4, 1.0, -0.1), &i),
            Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.3, -1.0, -0.7), &i),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-0.6, 0.3, 1.0), &i),
            Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.4, 0.4, -1.0), &i),
            -Vector::z_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(0.3, 0.4, 1.0), &i),
            Vector::z_axis()
        );

        assert_approx_eq!(
            c.normal_at(&Point::new(1.0, 1.0, 1.0), &i),
            Vector::x_axis()
        );
        assert_approx_eq!(
            c.normal_at(&Point::new(-1.0, -1.0, -1.0), &i),
            -Vector::x_axis()
        );
    }

    #[test]
    fn the_bounding_box_of_a_cube() {
        let c = Cube::new();

        assert_approx_eq!(
            c.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -1.0, -1.0),
                Point::new(1.0, 1.0, 1.0)
            )
        );
    }
}
