use std::ops::{Deref, DerefMut};

use derive_new::new;

use crate::{
    math::{Point, Ray, Vector},
    util::{approx::FLOAT_EPSILON, float::Float},
    Object,
};

/// A trait that `Object`s need to implement if they can be intersected in a
/// scene, returns a list of the intersection points.
pub trait Intersectable<T: Float> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>>;
    fn normal_at(&self, point: &Point<T>) -> Vector<T>;
}

/// A list of intersection point (t values) where intersections occur for a
/// given object.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct IntersectionPoints<T: Float>(Vec<T>);

impl<T: Float> From<Vec<T>> for IntersectionPoints<T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec)
    }
}

impl<T: Float> Deref for IntersectionPoints<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An `Intersection` stores both the t value of the intersection but also a
/// reference to the object that was intersected.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Intersection<'a, T: Float> {
    pub object: &'a Object<T>,
    pub t: T,
}

impl<'a, T: Float> Intersection<'a, T> {
    pub fn prepare_computations(
        &self,
        ray: &Ray<T>,
        intersections: &IntersectionList<T>,
    ) -> Computations<'a, T> {
        let point = ray.position(self.t);
        let eye = -ray.direction;
        let mut normal = self.object.normal_at(&point);

        let inside = if eye.dot(&normal) < T::zero() {
            normal = -normal;

            true
        } else {
            false
        };

        let over_point = point + normal * T::convert(FLOAT_EPSILON);

        let reflect = ray.direction.reflect(&normal);

        let mut containers: Vec<&Object<T>> = Vec::new();

        let mut n1 = T::infinity();
        let mut n2 = T::infinity();

        for intersection in intersections.iter() {
            if intersection == self {
                if containers.is_empty() {
                    n1 = T::one();
                } else {
                    n1 = containers.last().unwrap().material.refractive_index;
                };
            }

            if containers.contains(&intersection.object) {
                containers.retain(|object| *object != intersection.object);
            } else {
                containers.push(intersection.object);
            }

            if intersection == self {
                if containers.is_empty() {
                    n2 = T::one();
                } else {
                    n2 = containers.last().unwrap().material.refractive_index;
                }

                break;
            }
        }

        Computations::new(
            self.object,
            self.t,
            point,
            eye,
            normal,
            inside,
            over_point,
            reflect,
            n1,
            n2,
        )
    }
}

/// An `IntersectionList` is a simple wrapper around a vector of
/// `Intersection`s, it gives us type safety over using a plain Vec and makes it
/// obvious what we are doing.
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct IntersectionList<'a, T: Float>(Vec<Intersection<'a, T>>);

impl<'a, T: Float> IntersectionList<'a, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hit(&self) -> Option<&Intersection<T>> {
        let mut intersection = None;
        let mut smallest = T::infinity();

        for i in &self.0 {
            if i.t < smallest && i.t >= T::zero() {
                smallest = i.t;
                intersection = Some(i);
            }
        }

        intersection
    }
}

impl<'a, T: Float> From<Vec<Intersection<'a, T>>> for IntersectionList<'a, T> {
    fn from(vec: Vec<Intersection<'a, T>>) -> Self {
        Self(vec)
    }
}

impl<'a, T: Float> From<Intersection<'a, T>> for IntersectionList<'a, T> {
    fn from(intersection: Intersection<'a, T>) -> Self {
        Self::from(vec![intersection])
    }
}

impl<'a, T: Float> Deref for IntersectionList<'a, T> {
    type Target = Vec<Intersection<'a, T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Float> DerefMut for IntersectionList<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The `Computations` struct is a helper structure to store precomputed values
/// about an intersection.
#[allow(clippy::too_many_arguments)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, new)]
pub struct Computations<'a, T: Float> {
    pub object: &'a Object<T>,
    pub t: T,
    pub point: Point<T>,
    pub eye: Vector<T>,
    pub normal: Vector<T>,
    pub inside: bool,
    pub over_point: Point<T>,
    pub reflect: Vector<T>,
    pub n1: T,
    pub n2: T,
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use approx::*;

    use super::*;
    use crate::{math::Transform, Material};

    #[test]
    fn creating_intersection_points_from_a_vector() {
        let i = IntersectionPoints::from(vec![1.0, 4.5]);

        assert_eq!(i.len(), 2);
        assert_float_relative_eq!(i[0], 1.0);
        assert_float_relative_eq!(i[1], 4.5);
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let o = Object::default_sphere();

        let i = Intersection::new(&o, 3.5);

        assert_relative_eq!(*i.object, o);
        assert_float_relative_eq!(i.t, 3.5);
    }

    #[test]
    fn precomputing_the_state_when_an_intersection_occurs_on_the_outside() {
        let o = Object::default_sphere();
        let i = Intersection::new(&o, 4.0);

        let c = i.prepare_computations(
            &Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()),
            &IntersectionList::from(i),
        );

        assert_float_relative_eq!(c.t, i.t);
        assert_relative_eq!(c.object, i.object);
        assert_relative_eq!(c.point, Point::new(0.0, 0.0, -1.0));
        assert_relative_eq!(c.eye, -Vector::z_axis());
        assert_relative_eq!(c.normal, -Vector::z_axis());
        assert!(!c.inside);
    }

    #[test]
    fn precomputing_the_state_when_an_intersection_occurs_on_the_inside() {
        let o = Object::default_sphere();
        let i = Intersection::new(&o, 1.0);

        let c = i.prepare_computations(
            &Ray::new(Point::origin(), Vector::z_axis()),
            &IntersectionList::from(i),
        );

        assert_relative_eq!(c.point, Point::new(0.0, 0.0, 1.0));
        assert_relative_eq!(c.eye, -Vector::z_axis());
        assert_relative_eq!(c.normal, -Vector::z_axis());
        assert!(c.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let o = Object::new_sphere(
            Transform::from_translate(0.0, 0.0, 1.0),
            Material::default(),
        );
        let i = Intersection::new(&o, 5.0);

        let c = i.prepare_computations(
            &Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()),
            &IntersectionList::from(i),
        );

        assert!(c.over_point.z < -(FLOAT_EPSILON / 2.0));
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let o = Object::default_plane();
        let i = Intersection::new(&o, SQRT_2);

        let c = i.prepare_computations(
            &Ray::new(
                Point::new(0.0, 1.0, -1.0),
                Vector::new(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
            ),
            &IntersectionList::from(i),
        );

        assert_relative_eq!(
            c.reflect,
            Vector::new(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        )
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a =
            Object::new_glass_sphere(Transform::from_scale(2.0, 2.0, 2.0), 1.5);
        let b = Object::new_glass_sphere(
            Transform::from_translate(0.0, 0.0, -0.25),
            2.0,
        );
        let c = Object::new_glass_sphere(
            Transform::from_translate(0.0, 0.0, 0.25),
            2.5,
        );

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::z_axis());

        let i = IntersectionList::from(vec![
            Intersection::new(&a, 2.0),
            Intersection::new(&b, 2.75),
            Intersection::new(&c, 3.25),
            Intersection::new(&b, 4.75),
            Intersection::new(&c, 5.25),
            Intersection::new(&a, 6.0),
        ]);

        let test_values = |index: usize, n1: f64, n2: f64| {
            let c = i[index].prepare_computations(&r, &i);

            assert_relative_eq!(c.n1, n1);
            assert_relative_eq!(c.n2, n2);
        };

        test_values(0, 1.0, 1.5);
        test_values(1, 1.5, 2.0);
        test_values(2, 2.0, 2.5);
        test_values(3, 2.5, 2.5);
        test_values(4, 2.5, 1.5);
        test_values(5, 1.5, 1.0);
    }

    #[test]
    fn creating_a_new_intersection_list() {
        let o = Object::default_sphere();

        let mut list = IntersectionList::new();
        assert_eq!(list.len(), 0);

        list.push(Intersection::new(&o, 0.0));
        assert_eq!(list.len(), 1);
        assert_relative_eq!(*list[0].object, o);
        assert_float_relative_eq!(list[0].t, 0.0);
    }

    #[test]
    fn aggregating_intersections() {
        let o = Object::default_sphere();

        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, 2.0);

        let list = IntersectionList::from(vec![i1, i2]);

        assert_eq!(list.len(), 2);
        assert_relative_eq!(list[0].t, 1.0);
        assert_relative_eq!(list[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let o = Object::default_sphere();

        let i1 = Intersection::new(&o, 1.0);
        let i2 = Intersection::new(&o, 2.0);

        assert_eq!(IntersectionList::from(vec![i1, i2]).hit().unwrap(), &i1);
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let o = Object::default_sphere();

        let i1 = Intersection::new(&o, -1.0);
        let i2 = Intersection::new(&o, 1.0);

        assert_eq!(IntersectionList::from(vec![i1, i2]).hit().unwrap(), &i2);
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let o = Object::default_sphere();

        let i1 = Intersection::new(&o, -2.0);
        let i2 = Intersection::new(&o, -1.0);

        assert!(IntersectionList::from(vec![i1, i2]).hit().is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let o = Object::default_sphere();

        let i1 = Intersection::new(&o, 5.0);
        let i2 = Intersection::new(&o, 7.0);
        let i3 = Intersection::new(&o, -3.0);
        let i4 = Intersection::new(&o, 2.0);

        assert_eq!(
            IntersectionList::from(vec![i1, i2, i3, i4]).hit().unwrap(),
            &i4
        );
    }
}
