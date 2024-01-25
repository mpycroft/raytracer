use derive_new::new;

use super::Intersectable;
use crate::{
    intersection::{List, TList},
    math::{Point, Ray, Vector},
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
    #[must_use]
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

    #[must_use]
    fn normal_at(&self, point: &Point) -> Vector {
        *point - Point::origin()
    }
}

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

        let l = t.intersect(&r).unwrap().to_list(&o);

        assert_approx_eq!(Test::intersection_to_ray(&l), r);
    }

    #[test]
    fn normal_at_on_a_test_shape() {
        let t = Test::new();

        assert_approx_eq!(
            t.normal_at(&Point::new(1.0, 2.0, 3.0)),
            Vector::new(1.0, 2.0, 3.0)
        );
    }
}
