use std::f64::consts::PI;

use crate::math::{Point, Vector};

pub fn spherical_map(point: &Point) -> (f64, f64) {
    let theta = point.x.atan2(point.z);

    let radius = Vector::new(point.x, point.y, point.z).magnitude();

    let phi = (point.y / radius).acos();

    let raw_u = theta / (2.0 * PI);

    let u = 1.0 - (raw_u + 0.5);

    let v = 1.0 - phi / PI;

    (u, v)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn using_a_spherical_mapping_on_a_3d_point() {
        let test = |p, cu, cv| {
            let (u, v) = spherical_map(&p);

            assert_approx_eq!(u, cu);
            assert_approx_eq!(v, cv);
        };

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        test(Point::new(0.0, 0.0, -1.0), 0.0, 0.5);
        test(Point::new(1.0, 0.0, 0.0), 0.25, 0.5);
        test(Point::new(0.0, 0.0, 1.0), 0.5, 0.5);
        test(Point::new(-1.0, 0.0, 0.0), 0.75, 0.5);
        test(Point::new(0.0, 1.0, 0.0), 0.5, 1.0);
        test(Point::new(0.0, -1.0, 0.0), 0.5, 0.0);
        test(Point::new(sqrt_2_div_2, sqrt_2_div_2, 0.0), 0.25, 0.75);
    }
}
