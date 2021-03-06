use std::cell::Cell;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::FromPrimitive;

use crate::{
    intersect::{Intersectable, IntersectionPoints},
    math::{Point, Ray, Vector},
    util::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        float::Float,
    },
};

/// Test is a shape intended purely for testing functions on Object.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Test<T: Float> {
    pub ray: Cell<Option<Ray<T>>>,
}

impl<T: Float> Test<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Float> Intersectable<T> for Test<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>> {
        self.ray.set(Some(*ray));

        None
    }

    fn normal_at(&self, point: &Point<T>) -> Vector<T> {
        Vector::new(point.x, point.y, point.z)
    }
}

impl<T> AbsDiffEq for Test<T>
where
    T: Float + AbsDiffEq,
    T::Epsilon: FromPrimitive + Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self.ray.get(), other.ray.get()) {
            (None, None) => true,
            (Some(lhs), Some(rhs)) => lhs.abs_diff_eq(&rhs, epsilon),
            (_, _) => false,
        }
    }
}

impl<T> RelativeEq for Test<T>
where
    T: Float + RelativeEq,
    T::Epsilon: FromPrimitive + Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        FromPrimitive::from_f64(FLOAT_EPSILON).unwrap()
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

impl<T: Float> UlpsEq for Test<T>
where
    T: Float + UlpsEq,
    T::Epsilon: FromPrimitive + Copy,
{
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

    #[test]
    fn creating_a_test_object() {
        assert!(Test::<f64>::new().ray.get().is_none());
    }

    #[test]
    fn intersecting_a_test_object() {
        let r = Ray::new(Point::new(0.5, 1.0, 1.5), Vector::new(1.0, 1.0, 0.0));

        let t = Test::default();

        let i = t.intersect(&r);

        assert!(i.is_none());
        assert_relative_eq!(t.ray.get().unwrap(), r);
    }

    #[test]
    fn the_normal_of_a_test_object() {
        let t = Test::default();

        assert_relative_eq!(
            t.normal_at(&Point::new(2.0, 1.0, 0.0)),
            Vector::new(2.0, 1.0, 0.0)
        );
    }

    #[test]
    fn test_objects_are_approximately_equal() {
        let t1 = Test::<f64>::new();
        t1.ray.set(Some(Ray::new(Point::origin(), Vector::y_axis())));
        let t2 = Test::new();
        t2.ray.set(Some(Ray::new(Point::origin(), Vector::y_axis())));
        let t3 = Test::default();

        assert_abs_diff_eq!(t1, t2);
        assert_abs_diff_ne!(t1, t3);

        assert_relative_eq!(t1, t2);
        assert_relative_ne!(t1, t3);

        assert_ulps_eq!(t1, t2);
        assert_ulps_ne!(t1, t3);
    }
}
