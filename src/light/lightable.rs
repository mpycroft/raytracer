use enum_dispatch::enum_dispatch;

use crate::{math::Point, Colour, World};

/// A helper trait that represents the functions that can be called on `Light`s.
#[enum_dispatch(Light)]
pub trait Lightable {
    #[must_use]
    fn position(&self) -> Point;

    #[must_use]
    fn intensity(&self) -> Colour;

    #[must_use]
    fn intensity_at(&self, point: &Point, world: &World) -> f64;
}
