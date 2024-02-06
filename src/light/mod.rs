mod area;
mod lightable;
mod point;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use rand::Rng;

use self::area::Area;
pub use self::lightable::Lightable;
use crate::{
    math::{Point, Vector},
    Colour, World,
};

/// A `Light` represents some sort of light source in the scene.
#[derive(Clone, Copy, Debug)]
#[enum_dispatch]
pub enum Light {
    Area(Area),
    Point(point::Point),
}

impl Light {
    #[must_use]
    pub fn new_area(
        corner: Point,
        u: Vector,
        u_steps: u32,
        v: Vector,
        v_steps: u32,
        intensity: Colour,
    ) -> Self {
        Self::Area(Area::new(corner, u, u_steps, v, v_steps, intensity))
    }

    #[must_use]
    pub fn new_point(position: Point, intensity: Colour) -> Self {
        Self::Point(point::Point::new(position, intensity))
    }
}

impl ApproxEq for Light {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Area(lhs), Self::Area(rhs)) => lhs.approx_eq(rhs, margin),
            (Self::Point(lhs), Self::Point(rhs)) => lhs.approx_eq(rhs, margin),
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn comparing_lights() {
        let l1 = Light::new_point(Point::origin(), Colour::green());
        let l2 = Light::new_point(Point::origin(), Colour::green());
        let l3 = Light::new_point(Point::origin(), Colour::blue());
        let l4 = Light::new_area(
            Point::origin(),
            Vector::x_axis(),
            2,
            Vector::z_axis(),
            4,
            Colour::yellow(),
        );
        let l5 = Light::new_area(
            Point::origin(),
            Vector::x_axis(),
            2,
            Vector::z_axis(),
            4,
            Colour::yellow(),
        );

        assert_approx_eq!(l1, l2);

        assert_approx_ne!(l1, l3);

        assert_approx_eq!(l4, l5);

        assert_approx_ne!(l4, l1);
    }
}
