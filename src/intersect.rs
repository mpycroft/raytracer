use derive_more::{Constructor, Deref, DerefMut, From};
use float_cmp::{ApproxEq, F64Margin};

use super::Sphere;
use crate::math::Ray;

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList>;
}

/// An Intersection stores both the t value of the intersection in addition to a
/// reference to the object that was intersected.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub t: f64,
}

impl<'a> ApproxEq for Intersection<'a> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.object.approx_eq(*other.object, margin)
            && self.t.approx_eq(other.t, margin)
    }
}

/// A List is a simple wrapper around a vector of Intersections, it gives us
/// type safety over using a plain Vec and makes it obvious what we are doing.
#[derive(Clone, Debug, Deref, DerefMut, From)]
pub struct IntersectionList<'a>(Vec<Intersection<'a>>);

impl<'a> IntersectionList<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Find the intersection with the smallest positive t value.
    ///
    /// # Panics
    ///
    /// Will panic if any t values are NaN
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
    use crate::math::{float::*, Transformation};

    #[test]
    fn creating_an_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(&s, 1.5);

        assert_approx_eq!(i.object, s);
        assert_approx_eq!(i.t, 1.5);
    }

    #[test]
    fn creating_an_intersection_list() {
        let mut l = IntersectionList::new();
        assert_eq!(l.len(), 0);

        let s = Sphere::default();
        l.push(Intersection::new(&s, 1.2));

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, s);
        assert_approx_eq!(l[0].t, 1.2);

        let l = IntersectionList::default();
        assert_eq!(l.len(), 0);

        let l = IntersectionList::from(vec![
            Intersection::new(&s, 1.0),
            Intersection::new(&s, 2.0),
        ]);

        assert_eq!(l.len(), 2);
        assert_approx_eq!(l[0].t, 1.0);
        assert_approx_eq!(l[1].t, 2.0);
    }

    #[test]
    fn dereferencing_an_intersection_list() {
        let s = Sphere::default();
        let i1 = Intersection::new(&s, 1.5);
        let i2 = Intersection::new(&s, 2.5);

        let mut l = IntersectionList::from(vec![i1, i2]);

        assert_approx_eq!(l[0], i1);
        assert_approx_eq!(l[1], i2);

        l[0].t = 0.0;

        assert_approx_eq!(l[0].t, 0.0);
    }

    #[test]
    fn the_hit_when_all_intersections_are_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 1.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);

        let h = IntersectionList::from(vec![i1, i2]).hit();

        assert!(h.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);

        let h = IntersectionList::from(vec![i1, i2, i3, i4]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i4);
    }

    #[test]
    fn comparing_intersections() {
        let s1 = Sphere::default();
        let i1 = Intersection::new(&s1, 3.2);
        let i2 = Intersection::new(&s1, 3.2);
        let s2 = Sphere::new(Transformation::new().translate(1.0, 0.0, 0.0));
        let i3 = Intersection::new(&s2, 3.2);

        assert_approx_eq!(i1, i2);

        assert_approx_ne!(i1, i3);
    }
}
