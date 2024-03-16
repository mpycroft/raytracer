use derive_new::new;

use super::{Bounded, BoundingBox, Intersectable};
use crate::{
    intersection::{Intersection, List, TList},
    math::{float::impl_approx_eq, Point, Ray, Vector},
};
/// A `Test` is a shape intended purely for testing functions on `Object`.
#[derive(Clone, Copy, Debug, new)]
pub struct Test;

impl Test {
    #[must_use]
    pub fn intersection_to_ray(list: &List) -> Ray {
        assert_eq!(list.len(), 6);

        Ray::new(
            Point::new(list[0].t, list[1].t, list[2].t),
            Vector::new(list[3].t, list[4].t, list[5].t),
        )
    }
}

impl Intersectable for Test {
    fn intersect(&self, ray: &Ray) -> Option<TList> {
        Some(TList::from(vec![
            ray.origin.x,
            ray.origin.y,
            ray.origin.z,
            ray.direction.x,
            ray.direction.y,
            ray.direction.z,
        ]))
    }

    fn normal_at(&self, point: &Point, _intersection: &Intersection) -> Vector {
        *point - Point::origin()
    }
}

impl Bounded for Test {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, 1.0),
        )
    }
}

impl_approx_eq!(&Test { true });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Object};

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn intersecting_a_test_shape() {
        let t = Test::new();

        let r = Ray::new(Point::new(1.0, 2.0, 1.0), Vector::x_axis());

        let o = Object::test_builder().build();
        let l = t.intersect(&r).unwrap().into_list(&o);

        assert_approx_eq!(Test::intersection_to_ray(&l), r);
    }

    #[test]
    fn normal_at_on_a_test_shape() {
        let t = Test::new();

        let o = Object::test_builder().build();
        let i = Intersection::new(&o, 1.0);

        assert_approx_eq!(
            t.normal_at(&Point::new(1.0, 2.0, 3.0), &i),
            Vector::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn the_bounding_box_of_a_test() {
        let t = Test::new();

        assert_approx_eq!(
            t.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -1.0, -1.0),
                Point::new(1.0, 1.0, 1.0)
            )
        );
    }
}
