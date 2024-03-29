mod computations;
mod list;
mod t_list;
mod t_values;

use std::f64::EPSILON;

use float_cmp::{ApproxEq, F64Margin};

pub use self::{
    computations::Computations, list::List, t_list::TList, t_values::TValues,
};
use crate::{
    math::{float::approx_eq, Ray},
    Object,
};

/// An `Intersection` stores both the t value of the intersection in addition to a
/// reference to the object that was intersected. Optionally it holds the u and
/// v values that the intersection occurred at.
#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub t: f64,
    pub u_v: Option<(f64, f64)>,
}

impl<'a> Intersection<'a> {
    #[must_use]
    pub const fn new(object: &'a Object, t: f64) -> Self {
        Self { object, t, u_v: None }
    }

    #[must_use]
    pub const fn new_with_u_v(
        object: &'a Object,
        t: f64,
        u: f64,
        v: f64,
    ) -> Self {
        Self { object, t, u_v: Some((u, v)) }
    }

    #[must_use]
    pub fn prepare_computations(
        &self,
        ray: &Ray,
        intersections: &List,
    ) -> Computations {
        let point = ray.position(self.t);

        let eye = -ray.direction;
        let mut normal = self.object.normal_at(&point, self);

        let inside = if normal.dot(&eye) < 0.0 {
            normal *= -1.0;
            true
        } else {
            false
        };

        let mut container = Vec::<&Object>::new();

        let mut n1 = f64::NAN;
        let mut n2 = f64::NAN;

        for intersection in intersections.iter() {
            let is_hit = approx_eq!(intersection, *self);

            if is_hit {
                n1 = container.last().map_or_else(
                    || 1.0,
                    |object| object.material().refractive_index,
                );
            }

            if let Some(index) = container
                .iter()
                .position(|object| approx_eq!(object, intersection.object))
            {
                container.remove(index);
            } else {
                container.push(intersection.object);
            }

            if is_hit {
                n2 = container.last().map_or_else(
                    || 1.0,
                    |object| object.material().refractive_index,
                );

                break;
            }
        }

        Computations::new(
            self.object,
            self.t,
            point,
            point + normal * 100_000.0 * EPSILON,
            point - normal * 100_000.0 * EPSILON,
            eye,
            normal,
            inside,
            ray.direction.reflect(&normal),
            n1,
            n2,
        )
    }
}

impl<'a> ApproxEq for Intersection<'a> {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.object.approx_eq(other.object, margin)
            && self.t.approx_eq(other.t, margin)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::{
        math::{float::*, Point, Transformation, Vector},
        Material, Object,
    };

    #[test]
    fn creating_an_intersection() {
        let o = Object::test_builder().build();
        let i = Intersection::new(&o, 1.5);

        assert_approx_eq!(i.object, &o);
        assert_approx_eq!(i.t, 1.5);
        assert_eq!(i.u_v, None);

        let i = Intersection::new_with_u_v(&o, 0.6, 0.5, 0.4);

        assert_approx_eq!(i.object, &o);
        assert_approx_eq!(i.t, 0.6);
        assert_eq!(i.u_v, Some((0.5, 0.4)));
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());
        let o = Object::test_builder().build();
        let t = 4.0;
        let i = Intersection::new(&o, t);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(c.object, &o);
        assert_approx_eq!(c.t, t);
        assert_approx_eq!(c.point, Point::new(0.0, 0.0, -1.0));
        assert_approx_eq!(c.eye, -Vector::z_axis());
        assert_approx_eq!(c.normal, -Vector::z_axis());
        assert!(!c.inside);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::origin(), Vector::z_axis());
        let o = Object::test_builder().build();
        let t = 1.0;

        let i = Intersection::new(&o, t);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(c.object, &o);
        assert_approx_eq!(c.t, t);
        assert_approx_eq!(c.point, Point::new(0.0, 0.0, 1.0));
        assert_approx_eq!(c.eye, -Vector::z_axis());
        assert_approx_eq!(c.normal, -Vector::z_axis());
        assert!(c.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::test_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, 1.0))
            .build();

        let i = Intersection::new(&o, 5.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert!(c.over_point.z < -EPSILON / 2.0);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let o = Object::plane_builder().build();

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&o, SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            c.reflect,
            Vector::new(0.0, sqrt_2_div_2, sqrt_2_div_2)
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Object::sphere_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .material(Material::glass())
            .build();

        let mut m = Material::glass();
        m.refractive_index = 2.0;

        let b = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, -0.25))
            .material(m)
            .build();

        let mut m = Material::glass();
        m.refractive_index = 2.5;

        let c = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, 0.25))
            .material(m)
            .build();

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::z_axis());

        let l = List::from(vec![
            Intersection::new(&a, 2.0),
            Intersection::new(&b, 2.75),
            Intersection::new(&c, 3.25),
            Intersection::new(&b, 4.75),
            Intersection::new(&c, 5.25),
            Intersection::new(&a, 6.0),
        ]);

        let test = |idx: usize, n1: f64, n2: f64| {
            let c = l[idx].prepare_computations(&r, &l);

            assert_approx_eq!(c.n1, n1);
            assert_approx_eq!(c.n2, n2);
        };

        test(0, 1.0, 1.5);
        test(1, 1.5, 2.0);
        test(2, 2.0, 2.5);
        test(3, 2.5, 2.5);
        test(4, 2.5, 1.5);
        test(5, 1.5, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, 1.0))
            .material(Material::glass())
            .build();

        let i = Intersection::new(&o, 5.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert!(c.under_point.z > EPSILON / 2.0);
        assert!(c.point.z < c.under_point.z);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn preparing_the_normal_on_a_smooth_triangle() {
        let o = Object::triangle_builder(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::y_axis(),
            -Vector::x_axis(),
            Vector::x_axis(),
        )
        .build();

        let i = Intersection::new_with_u_v(&o, 1.0, 0.45, 0.25);

        let r = Ray::new(Point::new(-0.2, 0.3, -2.0), Vector::z_axis());

        let l = List::from(i);

        let c = i.prepare_computations(&r, &l);

        assert_approx_eq!(
            c.normal,
            Vector::new(-0.554_7, 0.832_05, 0.0),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn comparing_intersections() {
        let o1 = Object::test_builder().build();
        let i1 = Intersection::new(&o1, 3.2);
        let i2 = Intersection::new(&o1, 3.2);
        let o2 = Object::test_builder()
            .transformation(Transformation::new().translate(1.0, 0.0, 0.0))
            .build();
        let i3 = Intersection::new(&o2, 3.2);

        assert_approx_eq!(i1, i2);

        assert_approx_ne!(i1, i3);
    }
}
