use enum_dispatch::enum_dispatch;

use super::TList;
use crate::math::{Point, Ray, Vector};

/// A trait that `Shape`s need to implement if they can be intersected in a
/// scene, returns an optional `TList`.
#[enum_dispatch(Shape)]
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<TList>;

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector;
}
