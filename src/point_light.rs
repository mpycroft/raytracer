use derive_new::new;

use crate::{math::Point, util::float::Float, Colour};

/// A PointLight is a light source that has no size and radiates light in all
/// directions equally.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct PointLight<T: Float> {
    pub intensity: Colour<T>,
    pub position: Point<T>,
}

add_approx_traits!(PointLight<T> { intensity, position });

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let c = Colour::<f64>::white();
        let p = Point::origin();
        let l = PointLight::new(c, p);

        assert_relative_eq!(l.intensity, c);
        assert_relative_eq!(l.position, p);
    }

    #[test]
    fn point_lights_are_approximately_equal() {
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
