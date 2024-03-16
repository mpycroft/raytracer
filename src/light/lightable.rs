use enum_dispatch::enum_dispatch;
use rand::Rng;

use crate::{math::Point, Colour, World};

/// A helper trait that represents the functions that can be called on `Light`s.
#[enum_dispatch(Light)]
pub trait Lightable {
    fn positions<R: Rng>(&self, rng: &mut R) -> Vec<Point>;

    fn intensity(&self) -> Colour;

    fn intensity_at<R: Rng>(
        &self,
        point: &Point,
        world: &World,
        rng: &mut R,
    ) -> f64;
}
