mod test;

use float_cmp::{ApproxEq, F64Margin};
pub use test::Test;

use crate::{
    intersect::{Intersectable, IntersectionList},
    math::{Point, Ray, Vector},
};

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Debug)]
pub enum Shape {
    Test(Test),
}

impl Shape {
    #[must_use]
    pub fn new_test() -> Self {
        Self::Test(Test::new())
    }
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        match self {
            Shape::Test(test) => test.intersect(ray),
        }
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

impl ApproxEq for &Shape {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Shape::Test(lhs), Shape::Test(rhs)) => lhs.approx_eq(rhs, margin),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn comparing_shapes() {
        let s1 = Shape::new_test();
        let s2 = Shape::new_test();
        let s3 = Shape::new_test();
        let Shape::Test(t) = &s3;
        t.ray.set(Some(Ray::new(Point::origin(), Vector::x_axis())));

        assert_approx_eq!(s1, &s2);

        assert_approx_ne!(s1, &s3);
    }
}
