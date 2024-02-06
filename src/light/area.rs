use float_cmp::{ApproxEq, F64Margin};
use rand::prelude::*;

use super::Lightable;
use crate::{
    math::{Point, Vector},
    Colour, World,
};

#[derive(Clone, Copy, Debug)]
pub struct Area {
    corner: Point,
    u: Vector,
    u_steps: u32,
    v: Vector,
    v_steps: u32,
    samples: u32,
    intensity: Colour,
    position: Point,
}

impl Area {
    #[must_use]
    pub fn new(
        corner: Point,
        u: Vector,
        u_steps: u32,
        v: Vector,
        v_steps: u32,
        intensity: Colour,
    ) -> Self {
        let u_steps_float = f64::from(u_steps);
        let v_steps_float = f64::from(v_steps);

        let u = u / u_steps_float;
        let v = v / v_steps_float;

        Self {
            corner,
            u,
            u_steps,
            v,
            v_steps,
            samples: u_steps * v_steps,
            intensity,
            position: corner
                + u * u_steps_float / 2.0
                + v * v_steps_float / 2.0,
        }
    }

    #[must_use]
    fn point_on_light<R: Rng>(&self, u: u32, v: u32, rng: &mut R) -> Point {
        self.corner
            + self.u * (f64::from(u) + rng.gen_range(0.0..=1.0))
            + self.v * (f64::from(v) + rng.gen_range(0.0..=1.0))
    }
}

impl Lightable for Area {
    #[must_use]
    fn position(&self) -> Point {
        self.position
    }

    #[must_use]
    fn intensity(&self) -> Colour {
        self.intensity
    }

    #[must_use]
    fn intensity_at<R: Rng>(
        &self,
        point: &Point,
        world: &World,
        rng: &mut R,
    ) -> f64 {
        let mut intensity = 0.0;

        for v in 0..self.v_steps {
            for u in 0..self.u_steps {
                let position = self.point_on_light(u, v, rng);

                if !world.is_shadowed(&position, point) {
                    intensity += 1.0;
                }
            }
        }

        intensity / f64::from(self.samples)
    }
}

impl ApproxEq for Area {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.corner.approx_eq(other.corner, margin)
            && self.u.approx_eq(other.u, margin)
            && self.u_steps == other.u_steps
            && self.v.approx_eq(other.v, margin)
            && self.v_steps == other.v_steps
            && self.intensity.approx_eq(other.intensity, margin)
    }
}

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;
    use crate::{math::float::*, world::test_world};

    #[test]
    fn creating_an_area_light() {
        let a = Area::new(
            Point::origin(),
            Vector::new(2.0, 0.0, 0.0),
            4,
            Vector::z_axis(),
            2,
            Colour::white(),
        );

        assert_approx_eq!(a.corner, Point::origin());
        assert_approx_eq!(a.u, Vector::new(0.5, 0.0, 0.0));
        assert_eq!(a.u_steps, 4);
        assert_approx_eq!(a.v, Vector::new(0.0, 0.0, 0.5));
        assert_eq!(a.v_steps, 2);
        assert_eq!(a.samples, 8);
        assert_approx_eq!(a.intensity, Colour::white());
        assert_approx_eq!(a.position, Point::new(1.0, 0.0, 0.5));

        assert_approx_eq!(a.intensity(), Colour::white());
        assert_approx_eq!(a.position(), Point::new(1.0, 0.0, 0.5));
    }

    #[test]
    fn finding_a_single_point_on_an_area_light() {
        let a = Area::new(
            Point::origin(),
            Vector::new(2.0, 0.0, 0.0),
            4,
            Vector::z_axis(),
            2,
            Colour::white(),
        );

        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        assert_approx_eq!(
            a.point_on_light(0, 0, &mut r),
            Point::new(0.162_29, 0.0, 0.191_12),
            epsilon = 0.000_01
        );
        assert_approx_eq!(
            a.point_on_light(1, 0, &mut r),
            Point::new(0.679_81, 0.0, 0.005_73),
            epsilon = 0.000_01
        );
        assert_approx_eq!(
            a.point_on_light(0, 1, &mut r),
            Point::new(0.247_64, 0.0, 0.510_28),
            epsilon = 0.000_01
        );
        assert_approx_eq!(
            a.point_on_light(2, 0, &mut r),
            Point::new(1.428_62, 0.0, 0.42275),
            epsilon = 0.000_01
        );
        assert_approx_eq!(
            a.point_on_light(3, 1, &mut r),
            Point::new(1.647_44, 0.0, 0.537_12),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn area_light_intensity() {
        let w = test_world();

        let a = Area::new(
            Point::new(-0.5, -0.5, -5.0),
            Vector::x_axis(),
            2,
            Vector::y_axis(),
            2,
            Colour::white(),
        );

        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);

        assert_approx_eq!(
            a.intensity_at(&Point::new(0.0, 0.0, 2.0), &w, &mut r),
            0.0
        );
        assert_approx_eq!(
            a.intensity_at(&Point::new(1.0, -1.0, 2.0), &w, &mut r),
            0.5
        );
        assert_approx_eq!(
            a.intensity_at(&Point::new(1.5, 0.0, 2.0), &w, &mut r),
            0.5
        );
        assert_approx_eq!(
            a.intensity_at(&Point::new(1.25, 1.25, 3.0), &w, &mut r),
            0.75
        );
        assert_approx_eq!(
            a.intensity_at(&Point::new(0.0, 0.0, -2.0), &w, &mut r),
            1.0
        );
    }

    #[test]
    fn comparing_area_lights() {
        let a1 = Area::new(
            Point::origin(),
            Vector::x_axis(),
            2,
            Vector::y_axis(),
            4,
            Colour::white(),
        );
        let a2 = Area::new(
            Point::origin(),
            Vector::x_axis(),
            2,
            Vector::y_axis(),
            4,
            Colour::white(),
        );
        let a3 = Area::new(
            Point::origin(),
            Vector::x_axis(),
            3,
            Vector::y_axis(),
            4,
            Colour::white(),
        );

        assert_approx_eq!(a1, a2);

        assert_approx_ne!(a1, a3);
    }
}
