use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::TList,
    math::{Point, Ray, Vector},
};

/// A `Triangle` is a simple triangle defined by three vertices.
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    point1: Point,
    point2: Point,
    point3: Point,
    edge1: Vector,
    edge2: Vector,
    normal: Vector,
}

impl Triangle {
    #[must_use]
    pub fn new(point1: Point, point2: Point, point3: Point) -> Self {
        let edge1 = point2 - point1;
        let edge2 = point3 - point1;
        let normal = edge2.cross(&edge1).normalise();

        Self { point1, point2, point3, edge1, edge2, normal }
    }
}

impl Intersectable for Triangle {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<TList> {
        todo!()
    }

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

impl Bounded for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);

        let t = Triangle::new(p1, p2, p3);

        assert_approx_eq!(t.point1, p1);
        assert_approx_eq!(t.point2, p2);
        assert_approx_eq!(t.point3, p3);

        assert_approx_eq!(t.edge1, Vector::new(-1.0, -1.0, 0.0));
        assert_approx_eq!(t.edge2, Vector::new(1.0, -1.0, 0.0));

        assert_approx_eq!(t.normal, -Vector::z_axis());
    }
}
