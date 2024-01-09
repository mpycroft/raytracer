use std::f64::{EPSILON, INFINITY};

use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{Point, Ray, Vector},
};

/// A `Cube` is an axis aligned cube of size 2 (-1.0..1.0) on each axis.
#[derive(Clone, Copy, Debug)]
pub struct Cube;

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let min_numerator = -1.0 - origin;
    let max_numerator = 1.0 - origin;

    let (min, max) = if direction.abs() >= EPSILON {
        (min_numerator / direction, max_numerator / direction)
    } else {
        (min_numerator * INFINITY, max_numerator * INFINITY)
    };

    if min > max {
        return (max, min);
    }

    (min, max)
}

impl Intersectable for Cube {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        let (x_min, x_max) = check_axis(ray.origin.x, ray.direction.x);
        let (y_min, y_max) = check_axis(ray.origin.y, ray.direction.y);
        let (z_min, z_max) = check_axis(ray.origin.z, ray.direction.z);

        let min = x_min.max(y_min).max(z_min);
        let max = x_max.min(y_max).min(z_max);

        if min > max {
            return None;
        }

        Some(ListBuilder::new().add_t(min).add_t(max))
    }

    fn normal_at(&self, point: &Point) -> Vector {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::*, Vector},
        Object,
    };

    #[test]
    fn a_ray_intersects_a_cube() {
        let o = Object::default_test();

        let test = |r: &Ray, t1: f64, t2: f64| {
            let l = Cube.intersect(r).unwrap().object(&o).build();

            assert_approx_eq!(l[0].t, t1);
            assert_approx_eq!(l[1].t, t2);
        };

        test(&Ray::new(Point::new(5.0, 0.5, 0.0), -Vector::x_axis()), 4.0, 6.0);
        test(&Ray::new(Point::new(-5.0, 0.5, 0.0), Vector::x_axis()), 4.0, 6.0);
        test(&Ray::new(Point::new(0.5, 5.0, 0.0), -Vector::y_axis()), 4.0, 6.0);
        test(&Ray::new(Point::new(0.5, -5.0, 0.0), Vector::y_axis()), 4.0, 6.0);
        test(&Ray::new(Point::new(0.5, 0.0, 5.0), -Vector::z_axis()), 4.0, 6.0);
        test(&Ray::new(Point::new(0.5, 0.0, -5.0), Vector::z_axis()), 4.0, 6.0);

        test(&Ray::new(Point::new(0.0, 0.5, 0.0), Vector::z_axis()), -1.0, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let test = |r: &Ray| {
            let l = Cube.intersect(r);

            assert!(l.is_none());
        };

        test(&Ray::new(
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.267_3, 0.534_5, 0.801_8),
        ));
        test(&Ray::new(
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.801_8, 0.267_3, 0.534_5),
        ));
        test(&Ray::new(
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.534_5, 0.801_8, 0.267_3),
        ));
        test(&Ray::new(Point::new(2.0, 0.0, 2.0), -Vector::z_axis()));
        test(&Ray::new(Point::new(0.0, 2.0, 2.0), -Vector::y_axis()));
        test(&Ray::new(Point::new(2.0, 2.0, 0.0), -Vector::x_axis()));
    }

    #[test]
    fn the_normal_on_a_cube() {
        assert_approx_eq!(
            Cube.normal_at(&Point::new(1.0, 0.5, -0.8)),
            Vector::x_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(-1.0, -0.2, 0.9)),
            -Vector::x_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(-0.4, 1.0, -0.1)),
            Vector::y_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(0.3, -1.0, -0.7)),
            -Vector::y_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(-0.6, 0.3, 1.0)),
            Vector::z_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(0.4, 0.4, -1.0)),
            -Vector::z_axis()
        );

        assert_approx_eq!(
            Cube.normal_at(&Point::new(1.0, 1.0, 1.0)),
            Vector::x_axis()
        );
        assert_approx_eq!(
            Cube.normal_at(&Point::new(-1.0, -1.0, -1.0)),
            -Vector::x_axis()
        );
    }
}
