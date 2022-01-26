mod stripe;
#[cfg(test)]
mod test;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::FromPrimitive;
use paste::paste;

use self::stripe::Stripe;
#[cfg(test)]
use self::test::Test;
use crate::{
    math::{Point, Transform},
    util::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        float::Float,
    },
    Colour, Object,
};

/// A pattern that can be applied to a given object to change how it is rendered.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Pattern<T: Float> {
    transform: Transform<T>,
    pattern: Patterns<T>,
}

macro_rules! add_pattern_fns {
    ($pattern:ident ($($arg:ident: $type:ty),*)) => {
        paste! {
            pub fn [<new_ $pattern:snake>](
                transform: Transform<T>, $($arg: $type),*
            ) -> Self {
                Self::new(
                    transform, Patterns::$pattern($pattern::new($($arg),*))
                )
            }

            pub fn [<default_ $pattern:snake>]($($arg: $type),*) -> Self {
                Self::default(
                    Patterns::$pattern($pattern::new($($arg),*))
                )
            }
        }
    };
}

impl<T: Float> Pattern<T> {
    fn new(transform: Transform<T>, pattern: Patterns<T>) -> Self {
        Self { transform, pattern }
    }

    fn default(pattern: Patterns<T>) -> Self {
        Self::new(Transform::default(), pattern)
    }

    add_pattern_fns!(Stripe(a: Colour<T>, b: Colour<T>));

    #[cfg(test)]
    add_pattern_fns!(Test());

    pub fn pattern_at(
        &self,
        object: &Object<T>,
        point: &Point<T>,
    ) -> Colour<T> {
        self.pattern.pattern_at(point)
    }
}

add_approx_traits!(Pattern<T> { transform, pattern });

/// Trait that all Patterns must implement.
pub trait PatternAt<T: Float> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T>;
}

/// The set of patterns that we know how to render.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Patterns<T: Float> {
    Stripe(Stripe<T>),
    #[cfg(test)]
    Test(Test<T>),
}

impl<T: Float> PatternAt<T> for Patterns<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        match self {
            Patterns::Stripe(stripe) => stripe.pattern_at(point),
            #[cfg(test)]
            Patterns::Test(test) => test.pattern_at(point),
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
            #[cfg(test)]
            (Patterns::Test(lhs), Patterns::Test(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
            (_, _) => false,
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
            #[cfg(test)]
            (Patterns::Test(lhs), Patterns::Test(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
            (_, _) => false,
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
            #[cfg(test)]
            (Patterns::Test(lhs), Patterns::Test(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use crate::math::Angle;

    use super::*;

    #[test]
    fn create_a_new_stripe_pattern() {
        let t = Transform::<f64>::from_scale(1.0, 2.0, 2.0);
        let c1 = Colour::white();
        let c2 = Colour::black();

        let p = Pattern::new_stripe(t, c1, c2);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(p.pattern, Patterns::Stripe(Stripe::new(c1, c2)));
    }

    #[test]
    fn creating_a_default_stripe_pattern() {
        let c1 = Colour::new(0.0, 0.4, 0.9);
        let c2 = Colour::white();
        let p = Pattern::<f64>::default_stripe(c1, c2);

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(p.pattern, Patterns::Stripe(Stripe::new(c1, c2)));
    }

    #[test]
    fn create_a_new_test_pattern() {
        let t = Transform::from_rotate_x(Angle::from_degrees(30.0));

        let p = Pattern::new_test(t);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(p.pattern, Patterns::Test(Test::new()))
    }

    #[test]
    fn create_a_default_test_pattern() {
        let p = Pattern::<f64>::default_test();

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(p.pattern, Patterns::Test(Test::new()))
    }

    #[test]
    fn patterns_are_approximately_equal() {
        let p1 = Pattern::new_test(Transform::from_translate(1.0, 0.0, -2.0));
        let p2 = Pattern::new_test(Transform::from_translate(1.0, 0.0, -2.0));
        let p3 = Pattern::new_test(Transform::from_scale(2.0, 1.0, 1.0));

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
