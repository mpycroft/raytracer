mod group_helper;

use float_cmp::{ApproxEq, F64Margin};

pub use self::group_helper::{GroupHelper, GroupHelperBuilder};
use super::Object;
use crate::{bounding_box::BoundingBox, intersection::List, math::Ray};

/// A `Group` is a collection of `Object`s that can be treated as a single
/// entity.
#[derive(Clone, Debug)]
pub struct Group {
    objects: Vec<Object>,
    bounding_box: BoundingBox,
}

impl Group {
    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        if !self.bounding_box.is_intersected_by(ray) {
            return None;
        }

        let mut list = List::new();

        for object in &self.objects {
            if let Some(object_list) = object.intersect(ray) {
                list.extend(object_list.iter());
            };
        }

        if list.is_empty() {
            return None;
        }

        Some(list)
    }

    #[must_use]
    pub fn builder() -> GroupHelperBuilder<((), (Vec<Object>,))> {
        GroupHelper::builder()
    }
}

impl ApproxEq for &Group {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        if self.objects.len() != other.objects.len() {
            return false;
        }

        let margin = margin.into();

        for (lhs, rhs) in self.objects.iter().zip(&other.objects) {
            if !lhs.approx_eq(rhs, margin) {
                return false;
            }
        }

        true
    }
}
