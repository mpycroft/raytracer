use typed_builder::{Optional, TypedBuilder};

use super::{
    shapes::{Intersectable, Shapes},
    Bounded, BoundingBox, Includes, Object, Updatable,
};
use crate::{
    intersection::{Intersection, List},
    math::{
        float::{approx_eq, impl_approx_eq},
        Point, Ray, Transformable, Transformation, Vector,
    },
    Material,
};

#[allow(clippy::module_name_repetitions)]
pub(super) type ShapeBuilder = _ShapeBuilder<((), (), (), (Shapes,))>;

/// A `Shape` is a simple geometric shape, fixed around the origin.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(builder_type(name = _ShapeBuilder))]
#[builder(builder_method(vis = "pub(super)"))]
#[builder(build_method(vis = "", name = _build))]
pub struct Shape {
    #[builder(default = Transformation::new())]
    pub(super) transformation: Transformation,
    #[builder(default = Transformation::new(), setter(skip))]
    inverse_transformation: Transformation,
    #[builder(default = Material::default())]
    pub(super) material: Material,
    #[builder(default = true)]
    pub(super) casts_shadow: bool,
    #[allow(clippy::struct_field_names)]
    shape: Shapes,
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
    pub fn intersect<'a>(
        &'a self,
        ray: &Ray,
        object: &'a Object,
    ) -> Option<List<'a>> {
        let ray = self.to_object_space(ray);

        self.shape.intersect(&ray).map(|t_list| t_list.into_list(object))
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

impl Updatable for Shape {
    fn update_transformation(&mut self, transformation: &Transformation) {
        self.transformation = self.transformation.extend(transformation);
        self.inverse_transformation = self.transformation.invert();
    }

    fn replace_material(&mut self, material: &Material) {
        self.material = material.clone();
    }

    fn update_casts_shadow(&mut self, casts_shadow: bool) {
        self.casts_shadow = casts_shadow;
    }
}

impl Bounded for Shape {
    fn bounding_box(&self) -> BoundingBox {
        let bounding_box = self.shape.bounding_box();

        bounding_box.apply(&self.transformation)
    }
}

impl Includes for Shape {
    fn includes(&self, object: &Object) -> bool {
        if let Object::Shape(shape) = object {
            return approx_eq!(self, shape);
        }

        false
    }
}

impl_approx_eq!(&Shape { ref shape, transformation, ref material });

impl<T, M, S> _ShapeBuilder<(T, M, S, (Shapes,))>
where
    T: Optional<Transformation>,
    M: Optional<Material>,
    S: Optional<bool>,
{
    #[must_use]
    pub fn build(self) -> Object {
        let mut shape = self._build();

        shape.inverse_transformation = shape.transformation.invert();

        shape.into()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use paste::paste;

    use super::*;
    use crate::{
        math::{float::*, Angle},
        object::shapes::test::Test,
        Colour, Object,
    };

    #[test]
    fn creating_shapes() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let ti = t.invert();
        let m = Material::builder().pattern(Colour::red().into()).build();

        macro_rules! test_object {
            ($shape:ident($($args:expr $(,)?)*)) => {{
                paste! {
                    let s = Shapes::[<new_ $shape:lower>]($($args,)*);

                    let o = Object::[<$shape:lower _builder>]($($args,)*)
                        .transformation(t)
                        .material(m.clone())
                        .casts_shadow(false)
                        .build();

                    let Object::Shape(o) = o else { unreachable!() };

                    assert_approx_eq!(o.transformation, t);
                    assert_approx_eq!(o.inverse_transformation, ti);
                    assert_approx_eq!(o.material, &m);
                    assert!(!o.casts_shadow);
                    assert_approx_eq!(o.shape, &s);

                    let o = Object::[<$shape:lower _builder>]($($args,)*)
                        .build();

                    let Object::Shape(o) = o else { unreachable!() };

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

        let i = o.intersect(&r).unwrap();

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
    fn the_bounding_box_of_an_object() {
        let o = Object::sphere_builder()
            .transformation(
                Transformation::new()
                    .scale(0.5, 2.0, 4.0)
                    .translate(1.0, -3.0, 5.0),
            )
            .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(0.5, -5.0, 1.0),
                Point::new(1.5, -1.0, 9.0)
            )
        );
    }

    #[test]
    fn test_updating_a_shape() {
        let mut o = Object::sphere_builder().build();

        let t = Transformation::new().translate(1.0, 2.0, 3.0);

        o.update_transformation(&t);

        let m = Material::builder().ambient(1.0).diffuse(1.0).build();

        o.replace_material(&m);

        o.update_casts_shadow(false);

        let Object::Shape(s) = o else { unreachable!() };

        assert_approx_eq!(s.transformation, t);

        assert_approx_eq!(s.material, &m);

        assert!(!s.casts_shadow);
    }

    #[test]
    fn test_if_a_shape_includes_an_object() {
        let s = Object::sphere_builder().build();
        let p = Object::plane_builder().build();
        let g = Object::group_builder()
            .add_object(Object::plane_builder().build())
            .build();

        assert!(s.includes(&s));
        assert!(!s.includes(&p));
        assert!(!s.includes(&g));
    }

    #[test]
    fn comparing_shapes() {
        let o1 = Object::test_builder().build();
        let o2 = Object::test_builder().build();
        let o3 = Object::test_builder()
            .transformation(Transformation::new().scale(1.0, 2.0, 1.0))
            .build();

        assert_approx_eq!(o1, &o2);

        assert_approx_ne!(o1, &o3);
    }
}
