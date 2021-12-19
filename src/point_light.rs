use crate::{math::Point, Colour};

/// A PointLight is a light source that has no size and radiates light in all
/// directions equally.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub intensity: Colour,
    pub position: Point,
}

impl PointLight {
    pub fn new(intensity: Colour, position: Point) -> Self {
        Self { intensity, position }
    }
}

add_approx_traits!(PointLight { intensity, position });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn new() {
        let c = Colour::new(1.0, 1.0, 1.0);
        let p = Point::origin();
        let l = PointLight::new(c, p);

        assert_relative_eq!(l.intensity, c);
        assert_relative_eq!(l.position, p);
    }

    #[test]
    fn approx() {
        let l1 = PointLight::new(
            Colour::new(0.9, 0.5, 1.5),
            Point::new(0.1, -2.5, 3.65),
        );
        let l2 = PointLight::new(
            Colour::new(0.9, 0.5, 1.5),
            Point::new(0.1, -2.5, 3.65),
        );
        let l3 = PointLight::new(
            Colour::new(0.9, 0.500_1, 1.5),
            Point::new(0.09, -2.5, 3.65),
        );

        assert_abs_diff_eq!(l1, l2);
        assert_abs_diff_ne!(l1, l3);

        assert_relative_eq!(l1, l2);
        assert_relative_ne!(l1, l3);

        assert_ulps_eq!(l1, l2);
        assert_ulps_ne!(l1, l3);
    }
}
