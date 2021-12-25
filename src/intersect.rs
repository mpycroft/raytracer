use std::ops::{Deref, DerefMut};

use derive_more::Constructor;

use crate::{
    math::{Point, Ray, Vector},
    Sphere,
};

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList>;
}

/// An Intersection stores both the t value of the intersection but also a
/// reference to the object that was intersected.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Intersection<'a> {
    pub object: &'a Sphere,
    pub t: f64,
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: &Ray) -> Computations<'a> {
        let point = ray.position(self.t);
        let eye = -ray.direction;
        let mut normal = self.object.normal_at(&point);

        let inside = if eye.dot(&normal) < 0.0 {
            normal = -normal;

            true
        } else {
            false
        };

        Computations::new(self.object, self.t, point, eye, normal, inside)
    }
}

/// An IntersectionList is a simple wrapper around a vector of Intersections, it
/// gives us type safety over using a plain Vec and makes it obvious what we are
/// doing.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IntersectionList<'a>(Vec<Intersection<'a>>);

impl<'a> IntersectionList<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let mut intersection = None;
        let mut smallest = f64::INFINITY;

        for i in &self.0 {
            if i.t < smallest && i.t >= 0.0 {
                smallest = i.t;
                intersection = Some(i);
            }
        }

        intersection
    }
}

impl<'a> Default for IntersectionList<'a> {
    fn default() -> Self {
        Self::new()
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

/// The Computations struct is a helper structure to store precomputed values
/// about an intersection.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Computations<'a> {
    pub object: &'a Sphere,
    pub t: f64,
    pub point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn intersection_new() {
        let s = Sphere::default();

        let i = Intersection::new(&s, 3.5);

        assert_relative_eq!(*i.object, s);
        assert_float_relative_eq!(i.t, 3.5);
    }

    #[test]
    fn prepare_computations() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());
        let s = Sphere::default();
        let i = Intersection::new(&s, 4.0);

        let c = i.prepare_computations(&r);

        assert_float_relative_eq!(c.t, i.t);
        assert_relative_eq!(c.object, i.object);
        assert_relative_eq!(c.point, Point::new(0.0, 0.0, -1.0));
        assert_relative_eq!(c.eye, -Vector::z_axis());
        assert_relative_eq!(c.normal, -Vector::z_axis());
        assert!(!c.inside);

        let r = Ray::new(Point::origin(), Vector::z_axis());
        let i = Intersection::new(&s, 1.0);

        let c = i.prepare_computations(&r);

        assert_relative_eq!(c.point, Point::new(0.0, 0.0, 1.0));
        assert_relative_eq!(c.eye, -Vector::z_axis());
        assert_relative_eq!(c.normal, -Vector::z_axis());
        assert!(c.inside);
    }

    #[test]
    fn intersection_list_new() {
        let s = Sphere::default();

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

    #[test]
    fn hit() {
        let s = Sphere::default();

        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let list = IntersectionList::from(vec![i1, i2]);

        assert_eq!(list.hit().unwrap(), &i1);

        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 1.0);
        let list = IntersectionList::from(vec![i1, i2]);

        assert_eq!(list.hit().unwrap(), &i2);

        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        let list = IntersectionList::from(vec![i1, i2]);

        assert!(list.hit().is_none());

        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        let list = IntersectionList::from(vec![i1, i2, i3, i4]);

        assert_eq!(list.hit().unwrap(), &i4);
    }
}
