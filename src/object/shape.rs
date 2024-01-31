use typed_builder::{Optional, TypedBuilder};

use super::shapes::{Intersectable, Shapes};
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{
        float::impl_approx_eq, Point, Ray, Transformable, Transformation,
        Vector,
    },
    Material,
};

/// A `Shape` is a simple geometric shape, fixed around the origin.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(builder_method(vis = "pub(super)", name = _builder))]
#[builder(build_method(vis = "", name = _build))]
pub struct Shape {
    #[builder(default = Transformation::new())]
    transformation: Transformation,
    #[builder(default = Transformation::new(), setter(skip))]
    inverse_transformation: Transformation,
    #[builder(default = Material::default())]
    pub material: Material,
    #[builder(default = true)]
    pub casts_shadow: bool,
    pub shape: Shapes,
    #[builder(default = BoundingBox::default(), setter(skip))]
    pub bounding_box: BoundingBox,
}

impl Shape {
    #[must_use]
    pub fn to_object_space<T: Transformable>(&self, value: &T) -> T {
        value.apply(&self.inverse_transformation)
    }

    #[must_use]
    pub fn to_world_space<T: Transformable>(&self, value: &T) -> T {
        value.apply(&self.inverse_transformation.transpose())
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        if let Shapes::Group(group) = &self.shape {
            if !self.bounding_box.is_intersected_by(ray) {
                return None;
            }

            let mut list = List::new();

            for object in group.objects() {
                if let Some(object_list) = object.intersect(ray) {
                    list.extend(object_list.iter());
                };
            }

            if list.is_empty() {
                return None;
            }

            Some(list)
        } else {
            let ray = self.to_object_space(ray);

            self.shape.intersect(&ray, self)
        }
    }

    #[must_use]
    pub fn normal_at(
        &self,
        point: &Point,
        intersection: &Intersection,
    ) -> Vector {
        let object_point = self.to_object_space(point);

        let object_normal = self.shape.normal_at(&object_point, intersection);

        self.to_world_space(&object_normal).normalise()
    }
}

impl Bounded for Shape {
    #[must_use]
    fn bounding_box(&self) -> BoundingBox {
        let bounding_box = self.shape.bounding_box();

        bounding_box.apply(&self.transformation)
    }
}

impl_approx_eq!(&Shape { ref shape, transformation, ref material });

impl<T: Optional<Transformation>, M: Optional<Material>, S: Optional<bool>>
    ShapeBuilder<(T, M, S, (Shapes,))>
{
    #[must_use]
    pub fn build(self) -> Shape {
        let mut object = self._build();

        object.inverse_transformation = object.transformation.invert();

        if let Shapes::Group(group) = &mut object.shape {
            for child_object in group.iter_no_groups() {
                child_object.transformation =
                    child_object.transformation.extend(&object.transformation);
                child_object.inverse_transformation =
                    child_object.transformation.invert();
                child_object.bounding_box = child_object.bounding_box();
            }

            object.transformation = Transformation::new();
            object.inverse_transformation = Transformation::new();

            group.update_bounding_box();
        };

        object.bounding_box = object.bounding_box();

        object
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, PI, SQRT_2};

    use paste::paste;

    use super::*;
    use crate::{
        math::{float::*, Angle},
        object::shapes::test::Test,
        Colour, Object,
    };

    #[test]
    fn creating_an_object() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let ti = t.invert();
        let m = Material::builder().pattern(Colour::red().into()).build();

        /// Test the creation of objects using new_ and default_ functions.
        macro_rules! test_object {
            ($shape:ident($($args:expr $(,)?)*)) => {{
                paste! {
                    let s = Shapes::[<new_ $shape:lower>]($($args,)*);

                    let o = Object::[<$shape:lower _builder>]($($args,)*)
                        .transformation(t)
                        .material(m.clone())
                        .casts_shadow(false)
                        .build();

                    assert_approx_eq!(o.transformation, t);
                    assert_approx_eq!(o.inverse_transformation, ti);
                    assert_approx_eq!(o.material, &m);
                    assert!(!o.casts_shadow);
                    assert_approx_eq!(o.shape, &s);

                    let o = Object::[<$shape:lower _builder>]($($args,)*)
                        .build();

                    assert_approx_eq!(o.transformation, Transformation::new());
                    assert_approx_eq!(
                        o.inverse_transformation, Transformation::new()
                    );
                    assert_approx_eq!(o.material, &Material::default());
                    assert!(o.casts_shadow);
                    assert_approx_eq!(o.shape, &s);
                }
            }};
        }

        test_object!(Cone(0.0, 2.0, true));
        test_object!(Cube());
        test_object!(Cylinder(1.0, 2.0, false));
        test_object!(Plane());
        test_object!(Sphere());
        test_object!(Test());
    }

    #[test]
    fn intersecting_a_transformed_object_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::test_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let l = o.intersect(&r).unwrap();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );

        let o = Object::test_builder()
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build();

        let l = o.intersect(&r).unwrap();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let o = Object::test_builder()
            .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
            .build();

        let i = Intersection::new(&o, 2.5);

        assert_approx_eq!(
            o.normal_at(
                &Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &i
            ),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        let o = Object::test_builder()
            .transformation(
                Transformation::new()
                    .rotate_z(Angle(PI / 5.0))
                    .scale(1.0, 0.5, 1.0),
            )
            .build();

        let i = Intersection::new(&o, 2.5);

        let sqrt_2_div_d = SQRT_2 / 2.0;
        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, sqrt_2_div_d, -sqrt_2_div_d), &i),
            Vector::new(0.0, 0.970_14, -0.242_54),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::sphere_builder()
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let i = o.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].object, &o);
        assert_approx_eq!(i[1].object, &o);

        assert_approx_eq!(i[0].t, 3.0);
        assert_approx_eq!(i[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::sphere_builder()
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build();

        let i = o.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let o = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
            .build();

        let i = Intersection::new(&o, 0.0);

        assert_approx_eq!(
            o.normal_at(
                &Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &i
            ),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let o = Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .rotate_z(Angle::from_degrees(36.0))
                    .scale(1.0, 0.5, 1.0),
            )
            .build();

        let i = Intersection::new(&o, 2.1);

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, sqrt_2_div_2, -sqrt_2_div_2), &i),
            Vector::new(0.0, 0.970_14, -0.242_54),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn intersecting_an_empty_group() {
        let o = Object::group_builder(Vec::new()).build();

        assert!(o
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let s1 = Object::sphere_builder().build();
        let s2 = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, -3.0))
            .build();
        let s3 = Object::sphere_builder()
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build();

        let o = Object::group_builder(vec![s1.clone(), s2.clone(), s3]).build();

        let l = o
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .unwrap();
        assert_eq!(l.len(), 4);

        assert_approx_eq!(l[0].object, &s1);
        assert_approx_eq!(l[0].t, 4.0);
        assert_approx_eq!(l[1].object, &s1);
        assert_approx_eq!(l[1].t, 6.0);
        assert_approx_eq!(l[2].object, &s2);
        assert_approx_eq!(l[2].t, 1.0);
        assert_approx_eq!(l[3].object, &s2);
        assert_approx_eq!(l[3].t, 3.0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let o = Object::group_builder(vec![Object::sphere_builder()
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build()])
        .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
        .build();

        let l = o
            .intersect(&Ray::new(
                Point::new(10.0, 0.0, -10.0),
                Vector::z_axis(),
            ))
            .unwrap();
        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, 8.0);
        assert_approx_eq!(l[1].t, 12.0);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space_with_groups() {
        let o = Object::group_builder(vec![Object::group_builder(vec![
            Object::sphere_builder()
                .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
                .build(),
        ])
        .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
        .build()])
        .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
        .build();

        let Shapes::Group(g) = o.shape else { unreachable!() };
        let Shapes::Group(g) = &g.objects()[0].shape else { unreachable!() };
        let s = &g.objects()[0];

        assert_approx_eq!(
            s.to_object_space(&Point::new(-2.0, 0.0, -10.0)),
            Point::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space_with_groups() {
        let o = Object::group_builder(vec![Object::group_builder(vec![
            Object::sphere_builder()
                .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
                .build(),
        ])
        .transformation(Transformation::new().scale(1.0, 2.0, 3.0))
        .build()])
        .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
        .build();

        let Shapes::Group(g) = o.shape else { unreachable!() };
        let Shapes::Group(g) = &g.objects()[0].shape else { unreachable!() };
        let s = &g.objects()[0];

        let sqrt_3_div_3 = f64::sqrt(3.0) / 3.0;
        assert_approx_eq!(
            s.to_world_space(&Vector::new(
                sqrt_3_div_3,
                sqrt_3_div_3,
                sqrt_3_div_3
            ))
            .normalise(),
            Vector::new(0.285_71, 0.428_57, -0.857_14),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let o = Object::group_builder(vec![Object::group_builder(vec![
            Object::sphere_builder()
                .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
                .build(),
        ])
        .transformation(Transformation::new().scale(1.0, 2.0, 3.0))
        .build()])
        .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
        .build();

        let Shapes::Group(g) = o.shape else { unreachable!() };
        let Shapes::Group(g) = &g.objects()[0].shape else { unreachable!() };
        let s = &g.objects()[0];

        let o = Object::test_builder().build();

        let i = Intersection::new(&o, 1.2);

        assert_approx_eq!(
            s.normal_at(&Point::new(1.732_1, 1.154_7, -5.577_4), &i),
            Vector::new(0.285_7, 0.428_54, -0.857_16),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_bounding_box_of_an_object() {
        let o = Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .translate(1.0, 0.0, -1.0)
                    .scale(2.0, 2.0, 2.0),
            )
            .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(0.0, -2.0, -4.0),
                Point::new(4.0, 2.0, 0.0)
            )
        );
    }

    #[test]
    fn the_bounding_box_of_a_group() {
        let o =
            Object::group_builder(vec![Object::cone_builder(1.0, 3.0, true)
                .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                .build()])
            .transformation(Transformation::new().translate(1.0, 1.0, 0.0))
            .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(3.0, 3.0, 2.0),
                Point::new(7.0, 7.0, 6.0)
            )
        );
    }

    #[test]
    fn the_bounding_box_of_multiple_objects() {
        let o = Object::group_builder(vec![
            Object::sphere_builder()
                .transformation(Transformation::new().translate(3.0, 1.0, 3.0))
                .build(),
            Object::cube_builder()
                .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                .build(),
        ])
        .transformation(Transformation::new().translate(-1.0, 0.0, 0.0))
        .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(-3.0, -2.0, -2.0),
                Point::new(3.0, 2.0, 4.0)
            )
        );
    }

    #[test]
    fn comparing_objects() {
        let o1 = Object::test_builder().build();
        let o2 = Object::test_builder().build();
        let o3 = Object::test_builder()
            .transformation(Transformation::new().scale(1.0, 2.0, 1.0))
            .build();

        assert_approx_eq!(o1, &o2);

        assert_approx_ne!(o1, &o3);
    }
}
