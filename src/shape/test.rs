//! A `Test` is a shape intended purely for testing functions on `Object`.

use crate::{
    intersection::{List, ListBuilder},
    math::{Point, Ray, Vector},
};

#[must_use]
pub fn intersection_to_ray(list: &List) -> Ray {
    assert_eq!(list.len(), 6);

    Ray::new(
        Point::new(list[0].t, list[1].t, list[2].t),
        Vector::new(list[3].t, list[4].t, list[5].t),
    )
}

#[must_use]
#[allow(clippy::unnecessary_wraps)]
pub fn intersect<'a>(ray: &Ray) -> Option<ListBuilder<'a>> {
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

#[must_use]
pub fn normal_at(point: &Point) -> Vector {
    *point - Point::origin()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Object};

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn intersecting_a_test_shape() {
        let r = Ray::new(Point::new(1.0, 2.0, 1.0), Vector::x_axis());

        let i = intersect(&r);

        assert!(i.is_some());

        let o = Object::test_builder().build();
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(intersection_to_ray(&l), r);
    }

    #[test]
    fn normal_at_on_a_test_shape() {
        assert_approx_eq!(
            normal_at(&Point::new(1.0, 2.0, 3.0)),
            Vector::new(1.0, 2.0, 3.0)
        );
    }
}
