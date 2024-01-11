use derive_more::{Deref, DerefMut, From};

use super::Intersection;

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

impl<'a> From<Intersection<'a>> for List<'a> {
    fn from(value: Intersection<'a>) -> Self {
        Self::from(vec![value])
    }
}

impl<'a> Default for List<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{intersection::ListBuilder, math::float::*, Object};

    #[test]
    fn creating_an_intersection_list() {
        let mut l = List::new();
        assert_eq!(l.len(), 0);

        let o = Object::default_test();
        l.push(Intersection::new(&o, 1.2));

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, &o);
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

        assert_approx_eq!(h.object, &o);
        assert_approx_eq!(h.t, 2.0);
    }
}
