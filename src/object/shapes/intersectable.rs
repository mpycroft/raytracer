use enum_dispatch::enum_dispatch;

use crate::{
    intersection::{Intersection, TList},
    math::{Point, Ray, Vector},
};

/// A trait that `Shape`s need to implement if they can be intersected in a
/// scene, returns an optional `TList`.
#[enum_dispatch(Shapes)]
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<TList>;

    #[must_use]
    fn normal_at(&self, point: &Point, intersection: &Intersection) -> Vector;
}
