use derive_more::Constructor;

use crate::{
    math::{Point, Transformable, Transformation},
    Colour, Object,
};

/// A `Stripe` pattern alternates between two different `Colour`s as the x value
/// changes.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Stripe {
    transformation: Transformation,
    a: Colour,
    b: Colour,
}

impl Stripe {
    pub fn pattern_at(&self, object: &Object, point: &Point) -> Colour {
        let object_point = object.to_object_space(point);

        let pattern_point = object_point.apply(&self.transformation.invert());

        if pattern_point.x.floor() % 2.0 == 0.0 {
            return self.a;
        }

        self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::*, Transformation},
        Material,
    };

    #[test]
    fn creating_a_stripe_pattern() {
        let s = Stripe::new(
            Transformation::new(),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(s.a, Colour::white());
        assert_approx_eq!(s.b, Colour::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let o = Object::default_test();
        let s = Stripe::new(
            Transformation::new(),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(s.pattern_at(&o, &Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(0.0, 1.0, 0.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(0.0, 2.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let o = Object::default_test();
        let s = Stripe::new(
            Transformation::new(),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(s.pattern_at(&o, &Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(0.0, 0.0, 1.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(0.0, 0.0, 2.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let o = Object::default_test();
        let s = Stripe::new(
            Transformation::new(),
            Colour::white(),
            Colour::black(),
        );

        assert_approx_eq!(s.pattern_at(&o, &Point::origin()), Colour::white());

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(0.9, 0.0, 0.0)),
            Colour::white()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(1.0, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(-0.1, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(-1.0, 0.0, 0.0)),
            Colour::black()
        );

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(-1.1, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_an_object_transformation() {
        let o = Object::new_sphere(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material {
                pattern: Some(Stripe::new(
                    Transformation::new(),
                    Colour::white(),
                    Colour::black(),
                )),
                ..Default::default()
            },
        );

        let Some(s) = o.material.pattern else { unreachable!() };

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_a_pattern_transformation() {
        let o = Object::new_sphere(
            Transformation::new(),
            Material {
                pattern: Some(Stripe::new(
                    Transformation::new().scale(2.0, 2.0, 2.0),
                    Colour::white(),
                    Colour::black(),
                )),
                ..Default::default()
            },
        );

        let Some(s) = o.material.pattern else { unreachable!() };

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(1.5, 0.0, 0.0)),
            Colour::white()
        );
    }

    #[test]
    fn a_stripe_pattern_with_both_an_object_and_pattern_transformation() {
        let o = Object::new_sphere(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material {
                pattern: Some(Stripe::new(
                    Transformation::new().translate(0.5, 0.0, 0.0),
                    Colour::white(),
                    Colour::black(),
                )),
                ..Default::default()
            },
        );

        let Some(s) = o.material.pattern else { unreachable!() };

        assert_approx_eq!(
            s.pattern_at(&o, &Point::new(2.5, 0.0, 0.0)),
            Colour::white()
        );
    }
}
