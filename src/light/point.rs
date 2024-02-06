use derive_new::new;
use rand::prelude::*;

use super::Lightable;
use crate::{
    math::{self, float::impl_approx_eq},
    Colour, World,
};

/// A `Point` is a light source that has no size and radiates light in all
/// directions equally.
#[derive(Clone, Copy, Debug, new)]
pub struct Point {
    position: math::Point,
    intensity: Colour,
}

impl Lightable for Point {
    fn positions<R: Rng>(&self, _rng: &mut R) -> Vec<math::Point> {
        vec![self.position]
    }

    fn intensity(&self) -> Colour {
        self.intensity
    }

    fn intensity_at<R: Rng>(
        &self,
        point: &math::Point,
        world: &World,
        _rng: &mut R,
    ) -> f64 {
        if world.is_shadowed(&self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

impl_approx_eq!(Point { position, intensity });

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;
    use crate::{math::float::*, world::test_world};

    #[test]
    fn creating_a_point_light() {
        let l = Point::new(math::Point::origin(), Colour::green());

        assert_approx_eq!(l.position, math::Point::origin());
        assert_approx_eq!(l.intensity, Colour::green());
    }

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let w = test_world();

        let l = &w.lights[0];

        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        assert_approx_eq!(
            l.intensity_at(&math::Point::new(0.0, 1.000_01, 0.0), &w, &mut r),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::new(-1.000_01, 0.0, 0.0), &w, &mut r),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::new(0.0, 0.0, -1.000_01), &w, &mut r),
            1.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::new(0.0, 0.0, 1.000_01), &w, &mut r),
            0.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::new(1.000_01, 0.0, 0.0), &w, &mut r),
            0.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::new(0.0, -1.000_01, 0.0), &w, &mut r),
            0.0
        );
        assert_approx_eq!(
            l.intensity_at(&math::Point::origin(), &w, &mut r),
            0.0
        );
    }

    #[test]
    fn comparing_point_lights() {
        let l1 = Point::new(
            math::Point::new(0.5, 1.0, 2.0),
            Colour::new(0.3, 0.6, 0.8),
        );
        let l2 = Point::new(
            math::Point::new(0.5, 1.0, 2.0),
            Colour::new(0.3, 0.6, 0.8),
        );
        let l3 = Point::new(
            math::Point::new(0.5, 1.0, 2.000_1),
            Colour::new(0.3, 0.6, 0.8),
        );

        assert_approx_eq!(l1, l2);

        assert_approx_ne!(l1, l3);
    }
}
