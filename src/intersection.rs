use std::f64::EPSILON;

use derive_more::{Constructor, Deref, DerefMut, From};
use float_cmp::{ApproxEq, F64Margin};

use crate::{
    math::{Point, Ray, Vector},
    Object,
};

/// A trait that `Object`s need to implement if they can be intersected in a
/// scene, returns an optional `ListBuilder` for constructing a `List`.
pub trait Intersectable {
    #[must_use]
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>>;

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

        self.object.approx_eq(*other.object, margin)
            && self.t.approx_eq(other.t, margin)
    }
}

/// An `IntersectionList` is a simple wrapper around a vector of Intersections,
/// it gives us type safety over using a plain Vec and makes it obvious what we
/// are doing.
#[derive(Clone, Debug, From, Deref, DerefMut)]
pub struct List<'a>(Vec<Intersection<'a>>);

impl<'a> List<'a> {
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

impl<'a> Default for List<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// `ListBuilder` provides a way to generate an intersection `List` when the
/// calculation of the t values is further down the call chain than when we know
/// what object is being intersected. We can append multiple t values then set
/// the `Object` later on and get a `List` containing an `Intersection` for each
/// t value with the appropriate object set.
pub struct ListBuilder<'a> {
    object: Option<&'a Object>,
    t: Vec<f64>,
}

impl<'a> ListBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self { object: None, t: Vec::new() }
    }

    #[must_use]
    pub fn object(mut self, object: &'a Object) -> Self {
        self.object = Some(object);

        self
    }

    #[must_use]
    pub fn add_t(mut self, t: f64) -> Self {
        self.t.push(t);

        self
    }

    /// Builds an intersection 'List' from a set of t values and a given object.
    /// There must be at least one t value.
    ///
    /// # Panics
    ///
    /// Will panic if no object was set or no t values were set.
    #[must_use]
    pub fn build(self) -> List<'a> {
        let object = self.object.expect(
            "Object reference not set when creating intersection List.",
        );

        assert!(
            !self.t.is_empty(),
            "No t values were added when creating intersection List."
        );

        self.t
            .iter()
            .map(|t| Intersection::new(object, *t))
            .collect::<Vec<Intersection<'a>>>()
            .into()
    }
}

impl<'a> Default for ListBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::*, Transformation},
        Material,
    };

    #[test]
    fn creating_an_intersection() {
        let o = Object::default_test();
        let i = Intersection::new(&o, 1.5);

        assert_approx_eq!(i.object, o);
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

        assert_approx_eq!(c.object, o);
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

        assert_approx_eq!(c.object, o);
        assert_approx_eq!(c.t, t);
        assert_approx_eq!(c.point, Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(c.eye, -Vector::z_axis());
        assert_approx_eq!(c.normal, -Vector::z_axis());
        assert!(c.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_test(
            Transformation::new().translate(0.0, 0.0, 1.0),
            Material::default(),
        );

        let i = Intersection::new(&o, 5.0);

        let c = i.prepare_computations(&r);

        assert!(c.over_point.z < -EPSILON / 2.0);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn creating_an_intersection_list() {
        let mut l = List::new();
        assert_eq!(l.len(), 0);

        let o = Object::default_test();
        l.push(Intersection::new(&o, 1.2));

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, o);
        assert_approx_eq!(l[0].t, 1.2);

        let l = List::default();
        assert_eq!(l.len(), 0);

        let l = List::from(vec![
            Intersection::new(&o, 1.0),
            Intersection::new(&o, 2.0),
        ]);

        assert_eq!(l.len(), 2);
        assert_approx_eq!(l[0].t, 1.0);
        assert_approx_eq!(l[1].t, 2.0);
    }

    #[test]
    fn creating_an_intersection_list_with_builder() {
        let o = Object::default_test();

        let b = ListBuilder::new().object(&o).add_t(1.0);

        let l = b.build();

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, o);
        assert_approx_eq!(l[0].t, 1.0);

        let l = ListBuilder::new()
            .object(&o)
            .add_t(1.0)
            .add_t(2.0)
            .add_t(-2.0)
            .build();

        assert_eq!(l.len(), 3);

        assert_approx_eq!(l[0].object, o);
        assert_approx_eq!(l[0].t, 1.0);

        assert_approx_eq!(l[1].object, o);
        assert_approx_eq!(l[1].t, 2.0);

        assert_approx_eq!(l[2].object, o);
        assert_approx_eq!(l[2].t, -2.0);
    }

    #[test]
    #[should_panic(
        expected = "Object reference not set when creating intersection List."
    )]
    fn intersection_list_builder_without_setting_object() {
        let _ = ListBuilder::new().add_t(1.0).build();
    }

    #[test]
    #[should_panic(
        expected = "No t values were added when creating intersection List."
    )]
    fn intersection_list_builder_without_adding_t_values() {
        let o = Object::default_test();

        let _ = ListBuilder::new().object(&o).build();
    }

    #[test]
    fn dereferencing_an_intersection_list() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 1.5);
        let i2 = Intersection::new(&o, 2.5);

        let mut l = List::from(vec![i1, i2]);

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

        let h = List::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, -1.0);

        let h = List::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let o = Object::default_test();
        let i1 = Intersection::new(&o, -2.0);
        let i2 = Intersection::new(&o, -1.0);

        let h = List::from(vec![i1, i2]).hit();

        assert!(h.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let o = Object::default_test();

        let h = ListBuilder::new()
            .object(&o)
            .add_t(5.0)
            .add_t(7.0)
            .add_t(-3.0)
            .add_t(2.0)
            .build()
            .hit();

        assert!(h.is_some());

        let h = h.unwrap();

        assert_approx_eq!(h.object, o);
        assert_approx_eq!(h.t, 2.0);
    }

    #[test]
    fn comparing_intersections() {
        let o1 = Object::default_test();
        let i1 = Intersection::new(&o1, 3.2);
        let i2 = Intersection::new(&o1, 3.2);
        let o2 = Object::new_test(
            Transformation::new().translate(1.0, 0.0, 0.0),
            Material::default(),
        );
        let i3 = Intersection::new(&o2, 3.2);

        assert_approx_eq!(i1, i2);

        assert_approx_ne!(i1, i3);
    }
}
