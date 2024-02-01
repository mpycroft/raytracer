use derive_new::new;

use crate::{
    math::{float::impl_approx_eq, Point},
    Colour, World,
};

/// A `PointLight` is a light source that has no size and radiates light in all
/// directions equally.
#[derive(Clone, Copy, Debug, new)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Colour,
}

impl PointLight {
    #[must_use]
    pub fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        if world.is_shadowed(&self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

impl_approx_eq!(PointLight { position, intensity });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, world::test_world};

    #[test]
    fn creating_a_point_light() {
        let l = PointLight::new(Point::origin(), Colour::green());

        assert_approx_eq!(l.position, Point::origin());
        assert_approx_eq!(l.intensity, Colour::green());
    }

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let w = test_world();

        let l =
            &PointLight::new(Point::new(-10.0, 10.0, -10.0), Colour::white());

        assert_approx_eq!(
            l.intensity_at(&Point::new(0.0, 1.000_01, 0.0), &w),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&Point::new(-1.000_01, 0.0, 0.0), &w),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&Point::new(0.0, 0.0, -1.000_01), &w),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&Point::new(0.0, 0.0, 1.000_01), &w),
            0.0
        );
        assert_approx_eq!(
            l.intensity_at(&Point::new(1.000_01, 0.0, 0.0), &w),
            0.0
        );
        assert_approx_eq!(
            l.intensity_at(&Point::new(0.0, -1.000_01, 0.0), &w),
            0.0
        );
        assert_approx_eq!(l.intensity_at(&Point::origin(), &w), 0.0);
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
