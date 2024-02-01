use std::vec::IntoIter;

use derive_more::{Deref, DerefMut, From};

use super::Intersection;

/// A `List` is a simple wrapper around a vector of `Intersection`s, it gives us
/// type safety over using a plain Vec and makes it obvious what we are doing.
#[derive(Clone, Debug, From, Deref, DerefMut)]
pub struct List<'a>(Vec<Intersection<'a>>);

impl<'a> List<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Find the intersection with the smallest positive t value. Assumes the
    /// list of intersections is not sorted.
    ///
    /// This function should never panic. Our filter of > 0.0 removes any NaN
    /// values and +-Inf return orderings when compared.
    #[must_use]
    pub fn hit(&self) -> Option<Intersection<'a>> {
        self.0
            .iter()
            .filter(|val| val.t > 0.0)
            .min_by(|a, b| {
                a.t.partial_cmp(&b.t).unwrap_or_else(|| unreachable!())
            })
            .copied()
    }

    #[must_use]
    pub fn into_iter(self) -> IntoIter<Intersection<'a>> {
        self.0.into_iter()
    }

    pub fn sort(&mut self) {
        self.sort_by(|a, b| {
            a.t.partial_cmp(&b.t).unwrap_or_else(|| {
                panic!(
                    "\
Failed to compare floating point values '{}' and '{}' when sorting \
intersection list.",
                    a.t, b.t
                )
            })
        });
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
    use std::f64::{INFINITY, NEG_INFINITY};

    use super::*;
    use crate::{math::float::*, Object};

    #[test]
    fn creating_a_list() {
        assert_eq!(List::new().len(), 0);

        assert_eq!(List::default().len(), 0);

        let o = Object::test_builder().build();

        assert_eq!(List::from(Intersection::new(&o, -1.1)).len(), 1);

        assert_eq!(
            List::from(vec![
                Intersection::new(&o, 1.0),
                Intersection::new(&o, 2.5)
            ])
            .len(),
            2
        );
    }

    #[test]
    fn adding_to_a_list() {
        let mut l = List::new();

        let o = Object::test_builder().build();

        l.push(Intersection::new(&o, 1.2));
        l.push(Intersection::new(&o, 3.5));
        l.push(Intersection::new(&o, 2.1));

        assert_eq!(l.len(), 3);
    }

    #[test]
    fn dereferencing_a_list() {
        let o = Object::test_builder().build();
        let i1 = Intersection::new(&o, 1.2);
        let i2 = Intersection::new(&o, 2.4);

        let mut l = List::from(vec![i1, i2]);

        assert_approx_eq!(l[0], i1);
        assert_approx_eq!(l[1], i2);

        l[0].t = 5.0;

        assert_approx_eq!(l[0].t, 5.0);
    }

    #[test]
    fn the_hit_when_all_intersections_are_positive() {
        let o = Object::test_builder().build();
        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, 2.0);

        let h = List::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_some_intersections_are_negative() {
        let o = Object::test_builder().build();
        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, -1.0);

        let h = List::from(vec![i1, i2]).hit();

        assert!(h.is_some());
        assert_approx_eq!(h.unwrap(), i1);
    }

    #[test]
    fn the_hit_when_all_intersections_are_negative() {
        let o = Object::test_builder().build();
        let i1 = Intersection::new(&o, -2.0);
        let i2 = Intersection::new(&o, -1.0);

        assert!(List::from(vec![i1, i2]).hit().is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let o = Object::test_builder().build();

        let h = List::from(vec![
            Intersection::new(&o, 5.0),
            Intersection::new(&o, 7.0),
            Intersection::new(&o, -3.0),
            Intersection::new(&o, 2.0),
        ])
        .hit()
        .unwrap();

        assert_approx_eq!(h.object, &o);
        assert_approx_eq!(h.t, 2.0);
    }

    #[test]
    fn the_hit_with_nan_and_inf() {
        let o = Object::test_builder().build();

        let h = List::from(vec![
            Intersection::new(&o, 5.0),
            Intersection::new(&o, f64::NAN),
            Intersection::new(&o, f64::INFINITY),
            Intersection::new(&o, 2.5),
            Intersection::new(&o, f64::NEG_INFINITY),
            Intersection::new(&o, -f64::NAN),
        ])
        .hit()
        .unwrap();

        assert_approx_eq!(h.object, &o);
        assert_approx_eq!(h.t, 2.5);
    }

    #[test]
    fn sorting_a_list() {
        let o = Object::test_builder().build();

        let mut l = List::from(vec![
            Intersection::new(&o, 1.0),
            Intersection::new(&o, 2.0),
            Intersection::new(&o, 3.6),
            Intersection::new(&o, INFINITY),
            Intersection::new(&o, -0.5),
            Intersection::new(&o, 2.5),
            Intersection::new(&o, NEG_INFINITY),
        ]);

        l.sort();

        assert_approx_eq!(l[0].t, NEG_INFINITY);
        assert_approx_eq!(l[1].t, -0.5);
        assert_approx_eq!(l[2].t, 1.0);
        assert_approx_eq!(l[3].t, 2.0);
        assert_approx_eq!(l[4].t, 2.5);
        assert_approx_eq!(l[5].t, 3.6);
        assert_approx_eq!(l[6].t, INFINITY);
    }

    #[test]
    #[should_panic(expected = "\
Failed to compare floating point values 'NaN' and '0' when sorting \
intersection list.")]
    fn sorting_with_nan() {
        let o = Object::test_builder().build();

        let mut l = List::from(vec![
            Intersection::new(&o, 0.0),
            Intersection::new(&o, f64::NAN),
            Intersection::new(&o, 1.0),
            Intersection::new(&o, -f64::NAN),
        ]);

        l.sort();
    }
}
