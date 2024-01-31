use derive_more::{Deref, DerefMut, From};

use super::{Intersection, List, TValues};
use crate::Object;

/// A `TList` is a simple wrapper around a vector of `TValues`s, it gives us
/// type safety over using a plain Vec and makes it obvious what we are doing.
#[derive(Clone, Debug, From, Deref, DerefMut)]
pub struct TList(Vec<TValues>);

impl TList {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn into_list(self, object: &Object) -> List {
        List::from(
            self.iter()
                .map(|TValues { t, u_v }| match u_v {
                    Some((u, v)) => {
                        Intersection::new_with_u_v(object, *t, *u, *v)
                    }
                    None => Intersection::new(object, *t),
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl From<TValues> for TList {
    fn from(value: TValues) -> Self {
        Self(vec![value])
    }
}

impl From<f64> for TList {
    fn from(value: f64) -> Self {
        Self(vec![TValues::new(value)])
    }
}

impl From<Vec<f64>> for TList {
    fn from(value: Vec<f64>) -> Self {
        Self(value.iter().map(|t| TValues::new(*t)).collect())
    }
}

impl Default for TList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_t_list() {
        assert_eq!(TList::new().len(), 0);

        assert_eq!(TList::default().len(), 0);

        assert_eq!(TList::from(TValues::new(-1.1)).len(), 1);

        assert_eq!(
            TList::from(vec![TValues::new(1.0), TValues::new(2.5)]).len(),
            2
        );

        assert_eq!(TList::from(0.0).len(), 1);

        assert_eq!(TList::from(vec![1.0, 2.0, 3.0]).len(), 3);
    }

    #[test]
    fn adding_to_a_list() {
        let mut l = TList::new();

        l.push(TValues::new(1.2));
        l.push(TValues::new(3.5));
        l.push(TValues::new(2.1));

        assert_eq!(l.len(), 3);
    }

    #[test]
    fn dereferencing_a_list() {
        let mut l = TList::from(vec![1.2, 2.4]);

        assert_approx_eq!(l[0].t, 1.2);
        assert_approx_eq!(l[1].t, 2.4);

        l[0].t = 5.0;

        assert_approx_eq!(l[0].t, 5.0);
    }

    #[test]
    fn converting_t_list_to_a_list() {
        let mut t = TList::new();

        t.push(TValues::new(0.5));
        t.push(TValues::new_with_u_v(0.5, 0.9, 0.25));
        t.push(TValues::new(2.0));

        let o = Object::test_builder().build();

        let l = t.into_list(&o);

        assert_approx_eq!(l[0], Intersection::new(&o, 0.5));
        assert_approx_eq!(l[1], Intersection::new_with_u_v(&o, 0.5, 0.9, 0.25));
        assert_approx_eq!(l[2], Intersection::new(&o, 2.0));
    }
}
