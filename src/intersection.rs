use std::f64::EPSILON;

use derive_more::{Constructor, Deref, DerefMut, From};
use float_cmp::{ApproxEq, F64Margin};

use crate::{
    math::{Point, Ray, Vector},
    Object,
};

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList>;

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector;
}

/// An Intersection stores both the t value of the intersection in addition to a
/// reference to the object that was intersected.
/// An `Intersection` stores both the t value of the intersection in addition to
/// a reference to the `Object` that was intersected.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub t: f64,
}

/// The `Computations` struct is a helper structure to store precomputed values
/// about an intersection.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Computations<'a> {
    pub object: &'a Object,
    pub t: f64,
    pub point: Point,
    pub over_point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
}

impl<'a> Intersection<'a> {
    #[must_use]
    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position(self.t);

        let eye = -ray.direction;
        let mut normal = self.object.normal_at(&point);

        let inside = if normal.dot(&eye) < 0.0 {
            normal *= -1.0;
            true
        } else {
            false
        };

        let over_point = point + normal * 100_000.0 * EPSILON;

        Computations::new(
            self.object,
            self.t,
            point,
            over_point,
            eye,
            normal,
            inside,
        )
    }
}

impl<'a> ApproxEq for Intersection<'a> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.object.approx_eq(other.object, margin)
            && self.t.approx_eq(other.t, margin)
    }
}

/// An `IntersectionList` is a simple wrapper around a vector of Intersections,
/// it gives us type safety over using a plain Vec and makes it obvious what we
/// are doing.
#[derive(Clone, Debug, From, Deref, DerefMut)]
#[allow(clippy::module_name_repetitions)]
pub struct IntersectionList<'a>(Vec<Intersection<'a>>);

impl<'a> IntersectionList<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Find the intersection with the smallest positive t value. Assumes the
    /// list of intersections is not sorted.
    ///
    /// # Panics
    ///
    /// Will panic if any t values are NaN.
    #[must_use]
    pub fn hit(&self) -> Option<Intersection<'a>> {
        self.0
            .iter()
            .filter(|val| val.t > 0.0)
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
            .copied()
    }
}

impl<'a> Default for IntersectionList<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::*, Transformation},
        Material, Shape,
    };

    #[test]
    fn creating_an_intersection() {
        let o = Object::default_test();
        let i = Intersection::new(&o, 1.5);

        assert_approx_eq!(i.object, &o);
        assert_approx_eq!(i.t, 1.5);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());
        let o = Object::default_test();
        let t = 4.0;
        let i = Intersection::new(&o, t);

        let c = i.prepare_computations(&r);

        assert_approx_eq!(c.object, &o);
        assert_approx_eq!(c.t, t);
        assert_approx_eq!(c.point, Point::new(0.0, 0.0, -1.0));
        assert_approx_eq!(c.eye, -Vector::z_axis());
        assert_approx_eq!(c.normal, -Vector::z_axis());
        assert!(!c.inside);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::origin(), Vector::z_axis());
        let o = Object::default_test();
        let t = 1.0;

        let i = Intersection::new(&o, t);

        let c = i.prepare_computations(&r);

        assert_approx_eq!(c.object, &o);
        assert_approx_eq!(c.t, t);
        assert_approx_eq!(c.point, Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(c.eye, -Vector::z_axis());
        assert_approx_eq!(c.normal, -Vector::z_axis());
        assert!(c.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Object::new(
            Transformation::new().translate(0.0, 0.0, 1.0),
            Material::default(),
            Shape::new_test(),
        );

        let i = Intersection::new(&s, 5.0);

        let c = i.prepare_computations(&r);

        assert!(c.over_point.z < -EPSILON / 2.0);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn creating_an_intersection_list() {
        let mut l = IntersectionList::new();
        assert_eq!(l.len(), 0);

        let o = Object::default_test();
        l.push(Intersection::new(&o, 1.2));

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, &o);
        assert_approx_eq!(l[0].t, 1.2);

        let l = IntersectionList::default();
        assert_eq!(l.len(), 0);

        let l = IntersectionList::from(vec![
            Intersection::new(&o, 1.0),
            Intersection::new(&o, 2.0),
        ]);

        assert_eq!(l.len(), 2);
        assert_approx_eq!(l[0].t, 1.0);
        assert_approx_eq!(l[1].t, 2.0);
    }

    #[test]
    fn dereferencing_an_intersection_list() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 1.5);
        let i2 = Intersection::new(&o, 2.5);

        let mut l = IntersectionList::from(vec![i1, i2]);

        assert_approx_eq!(l[0], i1);
        assert_approx_eq!(l[1], i2);

        l[0].t = 0.0;

        assert_approx_eq!(l[0].t, 0.0);
    }

    #[test]
    fn the_hit_when_all_intersections_are_positive() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, 2.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, -1.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, -2.0);
        let i2 = Intersection::new(&o, -1.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 5.0);
        let i2 = Intersection::new(&o, 7.0);
        let i3 = Intersection::new(&o, -3.0);
        let i4 = Intersection::new(&o, 2.0);

        let h = IntersectionList::from(vec![i1, i2, i3, i4]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i4);
    }

    #[test]
    fn comparing_intersections() {
        let o1 = Object::default_test();
        let i1 = Intersection::new(&o1, 3.2);
        let i2 = Intersection::new(&o1, 3.2);
        let o2 = Object::new(
            Transformation::new().translate(1.0, 0.0, 0.0),
            Material::default(),
            Shape::new_test(),
        );
        let i3 = Intersection::new(&o2, 3.2);

        assert_approx_eq!(i1, i2);

        assert_approx_ne!(i1, i3);
    }
}
