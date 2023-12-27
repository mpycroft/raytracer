use crate::{
    intersection::{Intersectable, List, ListBuilder},
    math::{Point, Ray, Vector},
};

/// A `Test` is a shape intended purely for testing functions on `Object`.
#[derive(Clone, Copy, Debug)]
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
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        Some(
            ListBuilder::new()
                .add_t(ray.origin.x)
                .add_t(ray.origin.y)
                .add_t(ray.origin.z)
                .add_t(ray.direction.x)
                .add_t(ray.direction.y)
                .add_t(ray.direction.z),
        )
    }

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
        let t = Test;

        let r = Ray::new(Point::new(1.0, 2.0, 1.0), Vector::x_axis());

        let i = t.intersect(&r);

        assert!(i.is_some());

        let o = Object::default_test();
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(Test::intersection_to_ray(&l), r);
    }

    #[test]
    fn normal_at_on_a_test_shape() {
        assert_approx_eq!(
            Test.normal_at(&Point::new(1.0, 2.0, 3.0)),
            Vector::new(1.0, 2.0, 3.0)
        );
    }
}
