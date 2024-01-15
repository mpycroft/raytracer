use enum_dispatch::enum_dispatch;

use super::List;
use crate::math::{Point, Ray, Vector};

/// A trait that `Object`s need to implement if they can be intersected in a
/// scene, returns an optional `List`.
#[enum_dispatch(Shape)]
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<List>;

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector;
}
