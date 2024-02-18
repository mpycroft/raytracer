mod area;
mod lightable;
mod point;

use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use rand::Rng;
use serde::{Deserialize, Deserializer};

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

impl<'de> Deserialize<'de> for Light {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum Light {
            PointLight {
                at: Point,
                intensity: Colour,
            },
            AreaLight {
                corner: Point,
                #[serde(rename = "uvec")]
                u: Vector,
                #[serde(rename = "usteps")]
                u_steps: u32,
                #[serde(rename = "vvec")]
                v: Vector,
                #[serde(rename = "vsteps")]
                v_steps: u32,
                intensity: Colour,
            },
        }

        let light = Light::deserialize(deserializer)?;

        match light {
            Light::PointLight { at, intensity } => {
                Ok(Self::new_point(at, intensity))
            }
            Light::AreaLight { corner, u, u_steps, v, v_steps, intensity } => {
                Ok(Self::new_area(corner, u, u_steps, v, v_steps, intensity))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

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

    #[test]
    fn deserialize_point_light() {
        let l: Light = from_str(
            "\
at: [1, 2, 3]
intensity: [1, 0.5, 0]",
        )
        .unwrap();

        assert_approx_eq!(
            l,
            Light::new_point(
                Point::new(1.0, 2.0, 3.0),
                Colour::new(1.0, 0.5, 0.0)
            )
        );
    }

    #[test]
    fn deserialize_area_light() {
        let l: Light = from_str(
            "\
corner: [1, 2, 3]
uvec: [4, 0, 0]
usteps: 4
vvec: [0, 2, 0]
vsteps: 2
intensity: [0.5, 0.5, 0.8]",
        )
        .unwrap();

        assert_approx_eq!(
            l,
            Light::new_area(
                Point::new(1.0, 2.0, 3.0),
                Vector::new(4.0, 0.0, 0.0),
                4,
                Vector::new(0.0, 2.0, 0.0),
                2,
                Colour::new(0.5, 0.5, 0.8)
            )
        );
    }
}
