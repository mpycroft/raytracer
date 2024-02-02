mod point;

use float_cmp::{ApproxEq, F64Margin};

use crate::{math::Point, Colour, World};

/// A `Light` represents some sort of light source in the scene.
#[derive(Clone, Copy, Debug)]
pub enum Light {
    Point(point::Point),
}

impl Light {
    #[must_use]
    pub fn new_point(position: Point, intensity: Colour) -> Self {
        Self::Point(point::Point::new(position, intensity))
    }

    #[must_use]
    pub fn position(&self) -> Point {
        match self {
            Self::Point(point) => point.position,
        }
    }

    #[must_use]
    pub fn intensity(&self) -> Colour {
        match self {
            Self::Point(point) => point.intensity,
        }
    }

    #[must_use]
    pub fn intensity_at(&self, point: &Point, world: &World) -> f64 {
        match self {
            Self::Point(point_light) => point_light.intensity_at(point, world),
        }
    }
}

impl ApproxEq for Light {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Point(lhs), Self::Point(rhs)) => lhs.approx_eq(rhs, margin),
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

        assert_approx_eq!(l1, l2);

        assert_approx_ne!(l1, l3);
    }
}
