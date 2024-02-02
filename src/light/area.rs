use float_cmp::{ApproxEq, F64Margin};

use crate::{
    math::{Point, Vector},
    Colour,
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
    fn point_on_light(&self, u: u32, v: u32) -> Point {
        self.corner
            + self.u * (f64::from(u) + 0.5)
            + self.v * (f64::from(v) + 0.5)
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
    use super::*;
    use crate::math::float::*;

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

        assert_approx_eq!(a.point_on_light(0, 0), Point::new(0.25, 0.0, 0.25));
        assert_approx_eq!(a.point_on_light(1, 0), Point::new(0.75, 0.0, 0.25));
        assert_approx_eq!(a.point_on_light(0, 1), Point::new(0.25, 0.0, 0.75));
        assert_approx_eq!(a.point_on_light(2, 0), Point::new(1.25, 0.0, 0.25));
        assert_approx_eq!(a.point_on_light(3, 1), Point::new(1.75, 0.0, 0.75));
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
