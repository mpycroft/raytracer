use float_cmp::{ApproxEq, F64Margin};

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, TList, TValues},
    math::{
        float::{approx_eq, impl_approx_eq},
        Point, Ray, Vector,
    },
};

/// A `Triangle` is a simple triangle defined by three vertices.
#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    point1: Point,
    point2: Point,
    point3: Point,
    edge1: Vector,
    edge2: Vector,
    normal: Normal,
}

#[derive(Clone, Copy, Debug)]
enum Normal {
    Flat(Vector),
    Smooth(Vector, Vector, Vector),
}

impl Triangle {
    #[must_use]
    fn calculate_edges(
        point1: Point,
        point2: Point,
        point3: Point,
    ) -> (Vector, Vector) {
        (point2 - point1, point3 - point1)
    }

    #[must_use]
    pub fn new(point1: Point, point2: Point, point3: Point) -> Self {
        let (edge1, edge2) = Self::calculate_edges(point1, point2, point3);

        let normal = edge2.cross(&edge1).normalise();

        Self {
            point1,
            point2,
            point3,
            edge1,
            edge2,
            normal: Normal::Flat(normal),
        }
    }

    #[must_use]
    pub fn new_with_normals(
        point1: Point,
        point2: Point,
        point3: Point,
        normal1: Vector,
        normal2: Vector,
        normal3: Vector,
    ) -> Self {
        let (edge1, edge2) = Self::calculate_edges(point1, point2, point3);

        Self {
            point1,
            point2,
            point3,
            edge1,
            edge2,
            normal: Normal::Smooth(normal1, normal2, normal3),
        }
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

        Some(TList::from(TValues::new_with_u_v(t, u, v)))
    }

    #[must_use]
    fn normal_at(&self, _point: &Point, intersection: &Intersection) -> Vector {
        match self.normal {
            Normal::Flat(normal) => normal,
            Normal::Smooth(normal1, normal2, normal3) => {
                // The u and v values will always be set for triangles.
                let Some((u, v)) = intersection.u_v else { unreachable!() };

                normal2 * u + normal3 * v + normal1 * (1.0 - u - v)
            }
        }
    }
}

impl Bounded for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::from(vec![self.point1, self.point2, self.point3])
    }
}

// Edges are derived from the points, so no need to check them.
impl_approx_eq!(&Triangle { point1, point2, point3, normal });

impl ApproxEq for Normal {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Self::Flat(lhs), Self::Flat(rhs)) => lhs.approx_eq(rhs, margin),
            (
                Self::Smooth(lhs1, lhs2, lhs3),
                Self::Smooth(rhs1, rhs2, rhs3),
            ) => {
                lhs1.approx_eq(rhs1, margin)
                    && lhs2.approx_eq(rhs2, margin)
                    && lhs3.approx_eq(rhs3, margin)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Object};

    fn create_triangle() -> Triangle {
        Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        )
    }

    fn create_smooth_triangle() -> Triangle {
        Triangle::new_with_normals(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::y_axis(),
            -Vector::x_axis(),
            Vector::x_axis(),
        )
    }

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

        assert_approx_eq!(t.normal, Normal::Flat(-Vector::z_axis()));

        let n1 = Vector::y_axis();
        let n2 = -Vector::x_axis();
        let n3 = Vector::x_axis();

        let t = Triangle::new_with_normals(p1, p2, p3, n1, n2, n3);

        assert_approx_eq!(t.point1, p1);
        assert_approx_eq!(t.point2, p2);
        assert_approx_eq!(t.point3, p3);

        assert_approx_eq!(t.edge1, Vector::new(-1.0, -1.0, 0.0));
        assert_approx_eq!(t.edge2, Vector::new(1.0, -1.0, 0.0));

        assert_approx_eq!(t.normal, Normal::Smooth(n1, n2, n3));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = create_triangle();

        let o = Object::test_builder().build();
        let i = Intersection::new(&o, 0.0);

        let Normal::Flat(normal) = t.normal else { unreachable!() };

        assert_approx_eq!(t.normal_at(&Point::new(0.0, 0.5, 0.0), &i), normal);
        assert_approx_eq!(
            t.normal_at(&Point::new(-0.5, 0.75, 0.0), &i),
            normal
        );
        assert_approx_eq!(t.normal_at(&Point::new(0.5, 0.25, 0.0), &i), normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = create_triangle();

        assert!(
            t.intersect(&Ray::new(
                Point::new(0.0, -1.0, -2.0),
                Vector::y_axis()
            ),)
                .is_none()
        );
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = create_triangle();

        assert!(t
            .intersect(&Ray::new(Point::new(1.0, 1.0, -2.0), Vector::z_axis()),)
            .is_none());
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = create_triangle();

        assert!(t
            .intersect(&Ray::new(Point::new(-1.0, 1.0, -2.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = create_triangle();

        assert!(t
            .intersect(&Ray::new(Point::new(0.0, -1.0, -2.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let t = create_triangle();

        let l = t
            .intersect(&Ray::new(Point::new(0.0, 0.5, -2.0), Vector::z_axis()));

        assert!(l.is_some());

        let l = l.unwrap();

        assert_eq!(l.len(), 1);

        assert_approx_eq!(l[0].t, 2.0);
    }

    #[test]
    fn the_bounding_box_of_a_triangle() {
        let p1 = Point::origin();
        let p2 = Point::new(-1.0, -1.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 1.0);

        let t = Triangle::new(p1, p2, p3);

        assert_approx_eq!(
            t.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -1.0, 0.0),
                Point::new(0.0, 1.0, 1.0)
            )
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let t = create_smooth_triangle();

        let l = t.intersect(&Ray::new(
            Point::new(-0.2, 0.3, -2.0),
            Vector::z_axis(),
        ));

        assert!(l.is_some());

        let l = l.unwrap();

        assert_eq!(l.len(), 1);

        let (u, v) = l[0].u_v.unwrap();
        assert_approx_eq!(u, 0.45);
        assert_approx_eq!(v, 0.25);
    }

    #[test]
    fn a_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
        let t = create_smooth_triangle();

        let o = Object::test_builder().build();
        let i = Intersection::new_with_u_v(&o, 1.0, 0.45, 0.25);

        assert_approx_eq!(
            t.normal_at(&Point::origin(), &i).normalise(),
            Vector::new(-0.554_7, 0.832_05, 0.0),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn comparing_triangles() {
        let t1 = Triangle::new(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, -1.0),
        );
        let t2 = Triangle::new(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, -1.0),
        );
        let t3 = Triangle::new(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.000_05, 1.0, -1.0),
        );
        let t4 = Triangle::new_with_normals(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, -1.0),
            Vector::y_axis(),
            Vector::x_axis(),
            Vector::z_axis(),
        );
        let t5 = Triangle::new_with_normals(
            Point::new(-1.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(0.0, 1.0, -1.0),
            Vector::y_axis(),
            Vector::x_axis(),
            Vector::z_axis(),
        );

        assert_approx_eq!(t1, &t2);

        assert_approx_ne!(t1, &t3);

        assert_approx_eq!(t4, &t5);

        assert_approx_ne!(t4, &t3);
        assert_approx_ne!(t3, &t4);
    }
}
