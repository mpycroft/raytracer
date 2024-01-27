use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::TList,
    math::{float::approx_eq, Point, Ray, Vector},
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
        let dir_cross_e2 = ray.direction.cross(&self.edge2);
        let det = self.edge1.dot(&dir_cross_e2);

        if approx_eq!(det, 0.0) {
            return None;
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.point1;

        let u = f * p1_to_origin.dot(&dir_cross_e2);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let origin_cross_e1 = p1_to_origin.cross(&self.edge1);

        let v = f * ray.direction.dot(&origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let t = f * self.edge2.dot(&origin_cross_e1);

        Some(TList::from(t))
    }

    #[must_use]
    fn normal_at(&self, _point: &Point) -> Vector {
        self.normal
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

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert_approx_eq!(t.normal_at(&Point::new(0.0, 0.5, 0.0)), t.normal);
        assert_approx_eq!(t.normal_at(&Point::new(-0.5, 0.75, 0.0)), t.normal);
        assert_approx_eq!(t.normal_at(&Point::new(0.5, 0.25, 0.0)), t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert!(t
            .intersect(&Ray::new(Point::new(0.0, -1.0, -2.0), Vector::y_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert!(t
            .intersect(&Ray::new(Point::new(1.0, 1.0, -2.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert!(t
            .intersect(&Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert!(t
            .intersect(&Ray::new(Point::new(0.0, -1.0, -2.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let l = t
            .intersect(&Ray::new(Point::new(0.0, 0.5, -2.0), Vector::z_axis()));

        assert!(l.is_some());

        let l = l.unwrap();

        assert_eq!(l.len(), 1);

        assert_approx_eq!(l[0], 2.0);
    }
}
