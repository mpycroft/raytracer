use derive_more::{Deref, DerefMut, From};

use super::{Intersection, List};
use crate::Object;

/// A `TList` is a simple wrapper around a vector of f64 representing the
/// distances at which intersections occur.
#[derive(Clone, Debug, From, Deref, DerefMut)]
pub struct TList(Vec<f64>);

impl TList {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn to_list(self, object: &Object) -> List {
        self.0
            .iter()
            .map(|t| Intersection::new(object, *t))
            .collect::<Vec<Intersection>>()
            .into()
    }
}

impl From<f64> for TList {
    fn from(value: f64) -> Self {
        Self(vec![value])
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

        assert_eq!(TList::from(0.5).len(), 1);

        assert_eq!(TList::from(vec![1.0, 2.0]).len(), 2);
    }

    #[test]
    fn adding_to_a_t_list() {
        let mut l = TList::new();

        l.push(2.0);
        l.push(-1.0);

        assert_eq!(l.len(), 2);
    }

    #[test]
    fn dereferencing_a_t_list() {
        let mut l = TList::from(vec![1.0, 2.0]);

        assert_approx_eq!(l[0], 1.0);
        assert_approx_eq!(l[1], 2.0);

        l[0] = 0.0;

        assert_approx_eq!(l[0], 0.0);
    }

    #[test]
    fn converting_a_t_list_to_a_list() {
        let l = TList::from(vec![0.5, 1.0, 1.5]);

        let o = Object::test_builder().build();

        let l = l.to_list(&o);

        assert_eq!(l.len(), 3);

        assert_approx_eq!(l[0].object, &o);
        assert_approx_eq!(l[0].t, 0.5);
        assert_approx_eq!(l[1].object, &o);
        assert_approx_eq!(l[1].t, 1.0);
        assert_approx_eq!(l[2].object, &o);
        assert_approx_eq!(l[2].t, 1.5);
    }
}
