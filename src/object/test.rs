use std::cell::Cell;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    intersect::{Intersectable, IntersectionList},
    math::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        Ray,
    },
};

/// Test is a shape intended purely for testing functions on Object.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Test {
    pub ray: Cell<Option<Ray>>,
}

impl Test {
    pub fn new(ray: Option<Ray>) -> Self {
        Self { ray: Cell::new(ray) }
    }
}

impl Intersectable for Test {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        self.ray.set(Some(*ray));

        None
    }
}

impl AbsDiffEq for Test {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self.ray.get(), other.ray.get()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => lhs.abs_diff_eq(&rhs, epsilon),
            (_, _) => false,
        }
    }
}

impl RelativeEq for Test {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self.ray.get(), other.ray.get()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => {
                lhs.relative_eq(&rhs, epsilon, max_relative)
            }
            (_, _) => false,
        }
    }
}

impl UlpsEq for Test {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        match (self.ray.get(), other.ray.get()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => lhs.ulps_eq(&rhs, epsilon, max_ulps),
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;
    use crate::math::{Point, Vector};

    #[test]
    fn new() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::x_axis());

        assert_relative_eq!(Test::new(Some(r)).ray.get().unwrap(), r);
    }

    #[test]
    fn default() {
        assert!(Test::default().ray.get().is_none());
    }

    #[test]
    fn intersect() {
        let r = Ray::new(Point::new(0.5, 1.0, 1.5), Vector::new(1.0, 1.0, 0.0));

        let t = Test::default();

        let i = t.intersect(&r);

        assert!(i.is_none());
        assert_relative_eq!(t.ray.get().unwrap(), r);
    }

    #[test]
    fn approx() {
        let t1 = Test::new(Some(Ray::new(Point::origin(), Vector::y_axis())));
        let t2 = Test::new(Some(Ray::new(Point::origin(), Vector::y_axis())));
        let t3 = Test::default();

        assert_abs_diff_eq!(t1, t2);
        assert_abs_diff_ne!(t1, t3);

        assert_relative_eq!(t1, t2);
        assert_relative_ne!(t1, t3);

        assert_ulps_eq!(t1, t2);
        assert_ulps_ne!(t1, t3);
    }
}
