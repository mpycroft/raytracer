use enum_dispatch::enum_dispatch;

#[cfg(test)]
use super::Test;
use super::{
    util::impl_approx_eq_patterns, Blend, Checker, Gradient, Perturbed,
    RadialGradient, Ring, Solid, Stripe,
};

/// The set of all patterns we know how to render.
#[derive(Clone, Debug)]
#[enum_dispatch(PatternAt)]
pub enum Kind {
    Blend(Blend),
    Checker(Checker),
    Gradient(Gradient),
    Perturbed(Perturbed),
    RadialGradient(RadialGradient),
    Ring(Ring),
    Stripe(Stripe),
    Solid(Solid),
    #[cfg(test)]
    Test(Test),
}

impl_approx_eq_patterns! {
    Blend,
    Checker,
    Gradient,
    Perturbed,
    RadialGradient,
    Ring,
    Stripe,
    Solid,
    #[cfg(test)]
    Test
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Colour};

    #[test]
    fn comparing_kinds() {
        let k1 = Kind::Stripe(Stripe::new(
            Colour::black().into(),
            Colour::blue().into(),
        ));
        let k2 = Kind::Stripe(Stripe::new(
            Colour::black().into(),
            Colour::blue().into(),
        ));
        let k3 = Kind::Test(Test);

        assert_approx_eq!(k1, &k2);

        assert_approx_ne!(k1, &k3);
    }
}
