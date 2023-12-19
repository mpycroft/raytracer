use std::cell::Cell;

use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersect::{Intersectable, IntersectionList},
    math::{Point, Ray, Vector},
};

/// A `Test` is a shape intended purely for testing functions on `Object`.
#[derive(Clone, Debug)]
pub struct Test {
    pub ray: Cell<Option<Ray>>,
}

impl Test {
    pub fn new() -> Self {
        Self { ray: Cell::new(None) }
    }
}

impl Intersectable for Test {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        self.ray.replace(Some(*ray));

        None
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}

impl ApproxEq for &Test {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self.ray.get(), other.ray.get()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => lhs.approx_eq(rhs, margin),
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_test_shape() {
        let t = Test::new();
        assert!(t.ray.get().is_none());

        let t = Test::default();
        assert!(t.ray.get().is_none());
    }

    #[test]
    fn intersecting_a_test_shape() {
        let t = Test::new();

        let r = Ray::new(Point::new(1.0, 2.0, 1.0), Vector::x_axis());

        let i = t.intersect(&r);

        assert!(i.is_none());

        assert_approx_eq!(t.ray.get().unwrap(), r);
    }

    #[test]
    fn comparing_test_shapes() {
        let r = Ray::new(Point::origin(), Vector::y_axis());
        let t1 = Test::new();
        t1.ray.set(Some(r));
        let t2 = Test::new();
        t2.ray.set(Some(r));
        let t3 = Test::new();

        assert_approx_eq!(t1, &t2);

        assert_approx_ne!(t1, &t3);
    }
}
