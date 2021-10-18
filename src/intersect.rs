use crate::{math::Ray, Sphere};
use std::ops::{Deref, DerefMut};

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList>;
}

/// An Intersection stores both the t value of the intersection but also a
/// reference to the object that was intersected.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub t: f64,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a Sphere, t: f64) -> Self {
        Intersection { object, t }
    }
}

/// An IntersectionList is a simple wrapper around a vector of Intersections, it
/// gives us type safety over using a plain Vec and makes it obvious what we are
/// doing.
#[derive(Clone, Debug, PartialEq)]
pub struct IntersectionList<'a>(Vec<Intersection<'a>>);

impl<'a> IntersectionList<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<'a> From<Vec<Intersection<'a>>> for IntersectionList<'a> {
    fn from(vec: Vec<Intersection<'a>>) -> Self {
        Self(vec)
    }
}

impl<'a> Deref for IntersectionList<'a> {
    type Target = Vec<Intersection<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for IntersectionList<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn intersection_new() {
        let s = Sphere::new();

        let i = Intersection::new(&s, 3.5);

        assert_relative_eq!(*i.object, s);
        assert_float_relative_eq!(i.t, 3.5);
    }

    #[test]
    fn intersection_list_new() {
        let s = Sphere::new();

        let mut list = IntersectionList::new();
        assert_eq!(list.len(), 0);

        list.push(Intersection::new(&s, 0.0));
        assert_eq!(list.len(), 1);
        assert_relative_eq!(*list[0].object, s);
        assert_float_relative_eq!(list[0].t, 0.0);

        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);

        let list = IntersectionList::from(vec![i1, i2]);

        assert_eq!(list.len(), 2);
        assert_relative_eq!(list[0].t, 1.0);
        assert_relative_eq!(list[1].t, 2.0);
    }
}
