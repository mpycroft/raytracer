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

        println!("{x_min} {x_max}");
        println!("{y_min} {y_max}");
        println!("{z_min} {z_max}");

        let min = x_min.max(y_min).max(z_min);
        let max = x_max.min(y_max).min(z_max);

        Some(ListBuilder::new().add_t(min).add_t(max))
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
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
}
