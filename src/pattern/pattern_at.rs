use enum_dispatch::enum_dispatch;

#[cfg(test)]
use super::Test;
use super::{
    Blend, Checker, Gradient, Kind, Perturbed, RadialGradient, Ring, Solid,
    Stripe, TextureMap,
};
use crate::{math::Point, Colour};

/// A trait that all `Kind`s are required to implement, it takes a `Point` and
/// returns the `Colour` of the pattern at that point.
#[enum_dispatch]
#[allow(clippy::module_name_repetitions)]
pub trait PatternAt {
    fn pattern_at(&self, point: &Point) -> Colour;
}
