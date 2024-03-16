use enum_dispatch::enum_dispatch;

use super::{UvAlignCheck, UvChecker, UvPattern};
use crate::Colour;

/// A trait that all `UvPatterns`s are required to implement, it takes a `Point`
/// and returns the `Colour` of the pattern at that point.
#[enum_dispatch]
pub trait UvPatternAt {
    fn uv_pattern_at(&self, u: f64, v: f64) -> Colour;
}
