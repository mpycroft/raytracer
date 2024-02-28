use enum_dispatch::enum_dispatch;

use crate::Colour;

/// A trait that all `Kind`s are required to implement, it takes a `Point` and
/// returns the `Colour` of the pattern at that point.
#[enum_dispatch]
pub trait UvPatternAt {
    #[must_use]
    fn uv_pattern_at(&self, u: f64, v: f64) -> Colour;
}
