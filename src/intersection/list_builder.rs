use super::{Intersection, List};
use crate::Object;

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
    /// Will panic if no object was set or no t values were added.
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
    use crate::math::float::*;

    #[test]
    fn creating_an_intersection_list_with_builder() {
        let o = Object::default_test();

        let b = ListBuilder::new().object(&o).add_t(1.0);

        let l = b.build();

        assert_eq!(l.len(), 1);
        assert_approx_eq!(l[0].object, &o);
        assert_approx_eq!(l[0].t, 1.0);

        let l = ListBuilder::default()
            .object(&o)
            .add_t(1.0)
            .add_t(2.0)
            .add_t(-2.0)
            .build();

        assert_eq!(l.len(), 3);

        assert_approx_eq!(l[0].object, &o);
        assert_approx_eq!(l[0].t, 1.0);

        assert_approx_eq!(l[1].object, &o);
        assert_approx_eq!(l[1].t, 2.0);

        assert_approx_eq!(l[2].object, &o);
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
}
