use crate::{math::Ray, Sphere};

/// A trait that objects need to implement if they can be intersected in a
/// scene, returns a vector of intersection t values.
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn new() {
        let s = Sphere::new();

        let i = Intersection::new(&s, 3.5);

        assert_relative_eq!(*i.object, s);
        assert_float_relative_eq!(i.t, 3.5);

        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert_relative_eq!(xs[0].t, 1.0);
        assert_relative_eq!(xs[1].t, 2.0);
    }
}
