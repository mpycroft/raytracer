use enum_dispatch::enum_dispatch;

use super::ListBuilder;
use crate::math::{Point, Ray, Vector};

/// A trait that `Object`s need to implement if they can be intersected in a
/// scene, returns an optional `ListBuilder` for constructing a `List`.
#[enum_dispatch(Shape)]
pub trait Intersectable {
    #[must_use]
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>>;

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector;
}
