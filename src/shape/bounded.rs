use enum_dispatch::enum_dispatch;

use crate::bounding_box::BoundingBox;

/// The `Bounded` trait needs to be implemented for each `Shape` that can be
/// contained within a bounding box.
#[enum_dispatch(Shape)]
pub trait Bounded {
    #[must_use]
    fn bounding_box(&self) -> BoundingBox;
}
