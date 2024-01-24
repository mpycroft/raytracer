use derive_new::new;
use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersection::{Intersectable, TList},
    math::{Point, Ray, Vector},
    Object,
};

/// A `Group` is a collection of `Object`s that can be treated as a single
/// entity.
#[derive(Clone, Debug, new)]
pub struct Group(pub Vec<Object>);

impl Intersectable for Group {
    #[must_use]
    fn intersect(&self, _ray: &Ray) -> Option<TList> {
        unreachable!()
    }

    #[must_use]
    fn normal_at(&self, _point: &Point) -> Vector {
        unreachable!()
    }
}

impl ApproxEq for &Group {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        let margin = margin.into();

        for (lhs, rhs) in self.0.iter().zip(&other.0) {
            if !lhs.approx_eq(rhs, margin) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn comparing_groups() {
        let g1 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g2 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g3 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
            Object::plane_builder().build(),
        ]);

        assert_approx_eq!(g1, &g2);

        assert_approx_ne!(g1, &g3);
    }
}
