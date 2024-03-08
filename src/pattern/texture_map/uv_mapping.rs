use std::f64::consts::PI;

use serde::Deserialize;

use crate::math::{Point, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UvMapping {
    Cylindrical,
    Planar,
    Spherical,
}

fn cylindrical_mapping(point: &Point) -> (f64, f64) {
    let theta = point.x.atan2(point.z);

    let raw_u = theta / (2.0 * PI);

    let u = 1.0 - (raw_u + 0.5);
    let v = point.y.rem_euclid(1.0);

    (u, v)
}

fn planar_mapping(point: &Point) -> (f64, f64) {
    let u = point.x.rem_euclid(1.0);
    let v = point.z.rem_euclid(1.0);

    (u, v)
}

fn spherical_mapping(point: &Point) -> (f64, f64) {
    let theta = point.x.atan2(point.z);

    let radius = Vector::new(point.x, point.y, point.z).magnitude();

    let phi = (point.y / radius).acos();

    let raw_u = theta / (2.0 * PI);

    let u = 1.0 - (raw_u + 0.5);

    let v = 1.0 - phi / PI;

    (u, v)
}

impl UvMapping {
    pub fn get_u_v(self, point: &Point) -> (f64, f64) {
        match self {
            Self::Cylindrical => cylindrical_mapping(point),
            Self::Planar => planar_mapping(point),
            Self::Spherical => spherical_mapping(point),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use serde_yaml::from_str;

    use super::*;
    use crate::math::float::*;

    fn check_u_v<F>(p: Point, cu: f64, cv: f64, m: F)
    where
        F: Fn(&Point) -> (f64, f64),
    {
        let (u, v) = m(&p);

        assert_approx_eq!(u, cu);
        assert_approx_eq!(v, cv);
    }

    #[test]
    fn using_a_cylindrical_mapping_on_a_3d_point() {
        let sqrt_2_div_2 = SQRT_2 / 2.0;

        check_u_v(Point::new(0.0, 0.0, -1.0), 0.0, 0.0, cylindrical_mapping);
        check_u_v(Point::new(0.0, 0.5, -1.0), 0.0, 0.5, cylindrical_mapping);
        check_u_v(Point::new(0.0, 1.0, -1.0), 0.0, 0.0, cylindrical_mapping);
        check_u_v(
            Point::new(sqrt_2_div_2, 0.5, -sqrt_2_div_2),
            0.125,
            0.5,
            cylindrical_mapping,
        );
        check_u_v(Point::new(1.0, 0.5, 0.0), 0.25, 0.5, cylindrical_mapping);
        check_u_v(
            Point::new(sqrt_2_div_2, 0.5, sqrt_2_div_2),
            0.375,
            0.5,
            cylindrical_mapping,
        );
        check_u_v(Point::new(0.0, -0.25, 1.0), 0.5, 0.75, cylindrical_mapping);
        check_u_v(
            Point::new(-sqrt_2_div_2, 0.5, sqrt_2_div_2),
            0.625,
            0.5,
            cylindrical_mapping,
        );
        check_u_v(Point::new(-1.0, 1.25, 0.0), 0.75, 0.25, cylindrical_mapping);
        check_u_v(
            Point::new(-sqrt_2_div_2, 0.5, -sqrt_2_div_2),
            0.875,
            0.5,
            cylindrical_mapping,
        );
    }

    #[test]
    fn using_a_planar_mapping_on_a_3d_point() {
        check_u_v(Point::new(0.25, 0.0, 0.5), 0.25, 0.5, planar_mapping);
        check_u_v(Point::new(0.25, 0.0, -0.25), 0.25, 0.75, planar_mapping);
        check_u_v(Point::new(0.25, 0.5, -0.25), 0.25, 0.75, planar_mapping);
        check_u_v(Point::new(1.25, 0.0, 0.5), 0.25, 0.5, planar_mapping);
        check_u_v(Point::new(0.25, 0.0, -1.75), 0.25, 0.25, planar_mapping);
        check_u_v(Point::new(1.0, 0.0, -1.0), 0.0, 0.0, planar_mapping);
        check_u_v(Point::origin(), 0.0, 0.0, planar_mapping);
    }

    #[test]
    fn using_a_spherical_mapping_on_a_3d_point() {
        let sqrt_2_div_2 = SQRT_2 / 2.0;

        check_u_v(Point::new(0.0, 0.0, -1.0), 0.0, 0.5, spherical_mapping);
        check_u_v(Point::new(1.0, 0.0, 0.0), 0.25, 0.5, spherical_mapping);
        check_u_v(Point::new(0.0, 0.0, 1.0), 0.5, 0.5, spherical_mapping);
        check_u_v(Point::new(-1.0, 0.0, 0.0), 0.75, 0.5, spherical_mapping);
        check_u_v(Point::new(0.0, 1.0, 0.0), 0.5, 1.0, spherical_mapping);
        check_u_v(Point::new(0.0, -1.0, 0.0), 0.5, 0.0, spherical_mapping);
        check_u_v(
            Point::new(sqrt_2_div_2, sqrt_2_div_2, 0.0),
            0.25,
            0.75,
            spherical_mapping,
        );
    }

    #[test]
    fn get_u_v() {
        let (u, v) =
            UvMapping::Cylindrical.get_u_v(&Point::new(0.0, 0.0, -1.0));

        assert_approx_eq!(u, 0.0);
        assert_approx_eq!(v, 0.0);

        let (u, v) = UvMapping::Planar.get_u_v(&Point::new(0.25, 0.0, 0.5));

        assert_approx_eq!(u, 0.25);
        assert_approx_eq!(v, 0.5);

        let (u, v) = UvMapping::Spherical.get_u_v(&Point::new(0.0, 0.0, -1.0));

        assert_approx_eq!(u, 0.0);
        assert_approx_eq!(v, 0.5);
    }

    #[test]
    fn deserialize_mapping() {
        let m: UvMapping = from_str("cylindrical").unwrap();

        assert!(matches!(m, UvMapping::Cylindrical));

        let m: UvMapping = from_str("planar").unwrap();

        assert!(matches!(m, UvMapping::Planar));

        let m: UvMapping = from_str("spherical").unwrap();

        assert!(matches!(m, UvMapping::Spherical));
    }
}
