use derive_more::Constructor;

use crate::{
    math::{float::impl_approx_eq, Point},
    Colour,
};

/// A `PointLight` is a light source that has no size and radiates light in all
/// directions equally.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Colour,
}

impl_approx_eq!(PointLight { position, intensity });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_point_light() {
        let l = PointLight::new(Point::origin(), Colour::green());

        assert_approx_eq!(l.position, Point::origin());
        assert_approx_eq!(l.intensity, Colour::green());
    }

    #[test]
    fn comparing_point_lights() {
        let l1 = PointLight::new(
            Point::new(0.5, 1.0, 2.0),
            Colour::new(0.3, 0.6, 0.8),
        );
        let l2 = PointLight::new(
            Point::new(0.5, 1.0, 2.0),
            Colour::new(0.3, 0.6, 0.8),
        );
        let l3 = PointLight::new(
            Point::new(0.5, 1.0, 2.000_1),
            Colour::new(0.3, 0.6, 0.8),
        );

        assert_approx_eq!(l1, l2);

        assert_approx_ne!(l1, l3);
    }
}
