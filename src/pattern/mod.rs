mod checker;
mod gradient;
mod ring;
mod stripe;
#[cfg(test)]
mod test;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::FromPrimitive;
use paste::paste;

#[cfg(test)]
use self::test::Test;
use self::{checker::Checker, gradient::Gradient, ring::Ring, stripe::Stripe};
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

    add_pattern_fns!(Checker(a: Colour<T>, b: Colour<T>));
    add_pattern_fns!(Gradient(a: Colour<T>, b: Colour<T>));
    add_pattern_fns!(Ring(a: Colour<T>, b: Colour<T>));
    add_pattern_fns!(Stripe(a: Colour<T>, b: Colour<T>));

    #[cfg(test)]
    add_pattern_fns!(Test());

    pub fn pattern_at(
        &self,
        object: &Object<T>,
        point: &Point<T>,
    ) -> Colour<T> {
        let object_point = object.transform.invert().apply(point);
        let pattern_point = self.transform.invert().apply(&object_point);

        self.pattern.pattern_at(&pattern_point)
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
    Checker(Checker<T>),
    Gradient(Gradient<T>),
    Ring(Ring<T>),
    Stripe(Stripe<T>),
    #[cfg(test)]
    Test(Test<T>),
}

impl<T: Float> PatternAt<T> for Patterns<T> {
    fn pattern_at(&self, point: &Point<T>) -> Colour<T> {
        match self {
            Patterns::Checker(data) => data.pattern_at(point),
            Patterns::Gradient(data) => data.pattern_at(point),
            Patterns::Ring(data) => data.pattern_at(point),
            Patterns::Stripe(data) => data.pattern_at(point),
            #[cfg(test)]
            Patterns::Test(data) => data.pattern_at(point),
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
            (Patterns::Checker(lhs), Patterns::Checker(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
            (Patterns::Gradient(lhs), Patterns::Gradient(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
            (Patterns::Ring(lhs), Patterns::Ring(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
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
            (Patterns::Checker(lhs), Patterns::Checker(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
            (Patterns::Gradient(lhs), Patterns::Gradient(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
            (Patterns::Ring(lhs), Patterns::Ring(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
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
            (Patterns::Checker(lhs), Patterns::Checker(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
            (Patterns::Gradient(lhs), Patterns::Gradient(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
            (Patterns::Ring(lhs), Patterns::Ring(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
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

    use super::*;
    use crate::{math::Angle, Material};

    #[test]
    fn creating_a_new_checker_pattern() {
        let t = Transform::<f64>::new();
        let c1 = Colour::black();
        let c2 = Colour::green();

        let p = Pattern::new_checker(t, c1, c2);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(p.pattern, Patterns::Checker(Checker::new(c1, c2)));
    }

    #[test]
    fn creating_a_default_checker_pattern() {
        let c1 = Colour::red();
        let c2 = Colour::green();
        let p = Pattern::<f64>::default_checker(c1, c2);

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(p.pattern, Patterns::Checker(Checker::new(c1, c2)));
    }

    #[test]
    fn creating_a_new_gradient_pattern() {
        let t = Transform::<f64>::from_translate(-1.0, -1.5, -2.0);
        let c1 = Colour::black();
        let c2 = Colour::white();

        let p = Pattern::new_gradient(t, c1, c2);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(
            p.pattern,
            Patterns::Gradient(Gradient::new(c1, c2))
        );
    }

    #[test]
    fn creating_a_default_gradient_pattern() {
        let c1 = Colour::red();
        let c2 = Colour::blue();
        let p = Pattern::<f64>::default_gradient(c1, c2);

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(
            p.pattern,
            Patterns::Gradient(Gradient::new(c1, c2))
        );
    }

    #[test]
    fn creating_a_new_ring_pattern() {
        let t = Transform::<f64>::from_translate(0.0, 1.0, 0.5);
        let c1 = Colour::red();
        let c2 = Colour::white();

        let p = Pattern::new_ring(t, c1, c2);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(p.pattern, Patterns::Ring(Ring::new(c1, c2)));
    }

    #[test]
    fn creating_a_default_ring_pattern() {
        let c1 = Colour::black();
        let c2 = Colour::blue();
        let p = Pattern::<f64>::default_ring(c1, c2);

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(p.pattern, Patterns::Ring(Ring::new(c1, c2)));
    }

    #[test]
    fn creating_a_new_stripe_pattern() {
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
    fn creating_a_new_test_pattern() {
        let t = Transform::from_rotate_x(Angle::from_degrees(30.0));

        let p = Pattern::new_test(t);

        assert_relative_eq!(p.transform, t);
        assert_relative_eq!(p.pattern, Patterns::Test(Test::new()))
    }

    #[test]
    fn creating_a_default_test_pattern() {
        let p = Pattern::<f64>::default_test();

        assert_relative_eq!(p.transform, Transform::default());
        assert_relative_eq!(p.pattern, Patterns::Test(Test::new()))
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        assert_relative_eq!(
            Pattern::default_test().pattern_at(
                &Object::new_sphere(
                    Transform::from_scale(2.0, 2.0, 2.0),
                    Material::default(),
                ),
                &Point::new(2.0, 3.0, 4.0)
            ),
            Colour::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        assert_relative_eq!(
            Pattern::new_test(Transform::from_scale(2.0, 2.0, 2.0)).pattern_at(
                &Object::default_sphere(),
                &Point::new(2.0, 3.0, 4.0)
            ),
            Colour::new(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn a_pattern_with_both_an_object_and_pattern_transformation() {
        assert_relative_eq!(
            Pattern::new_test(Transform::from_translate(0.5, 1.0, 1.5))
                .pattern_at(
                    &Object::new_sphere(
                        Transform::from_scale(2.0, 2.0, 2.0),
                        Material::default(),
                    ),
                    &Point::new(2.5, 3.0, 3.5)
                ),
            Colour::new(0.75, 0.5, 0.25)
        );
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
