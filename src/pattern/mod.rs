mod stripe;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::FromPrimitive;

pub use self::stripe::Stripe;
use crate::{
    math::Point,
    util::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        float::Float,
    },
    Colour,
};

/// Trait that all Patterns must implement.
pub trait PatternAt<T: Float> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T>;
}

/// A pattern that can be applied to a given object to change how it is rendered.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Pattern<T: Float> {
    pub pattern: Patterns<T>,
}

impl<T: Float> PatternAt<T> for Pattern<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        self.pattern.pattern_at(point)
    }
}

add_approx_traits!(Pattern<T> { pattern });

/// The set of patterns that we know how to render.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Patterns<T: Float> {
    Stripe(Stripe<T>),
}

impl<T: Float> PatternAt<T> for Patterns<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        match self {
            Patterns::Stripe(stripe) => stripe.pattern_at(point),
        }
    }
}

impl<T> AbsDiffEq for Patterns<T>
where
    T: Float + AbsDiffEq,
    T::Epsilon: FromPrimitive + Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self, other) {
            (Patterns::Stripe(lhs), Patterns::Stripe(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
        }
    }
}

impl<T> RelativeEq for Patterns<T>
where
    T: Float + RelativeEq,
    T::Epsilon: FromPrimitive + Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self, other) {
            (Patterns::Stripe(lhs), Patterns::Stripe(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
        }
    }
}

impl<T> UlpsEq for Patterns<T>
where
    T: Float + UlpsEq,
    T::Epsilon: FromPrimitive + Copy,
{
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        match (self, other) {
            (Patterns::Stripe(lhs), Patterns::Stripe(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn patterns_are_approximately_equal() {
        let p1 = Patterns::<f64>::Stripe(Stripe::new(
            Colour::white(),
            Colour::blue(),
        ));
        let p2 = Patterns::<f64>::Stripe(Stripe::new(
            Colour::white(),
            Colour::blue(),
        ));
        let p3 = Patterns::<f64>::Stripe(Stripe::new(
            Colour::white(),
            Colour::new(0.0, 0.0, 1.000_005),
        ));

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
