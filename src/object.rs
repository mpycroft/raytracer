use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{
        float::impl_approx_eq, Point, Ray, Transformable, Transformation,
        Vector,
    },
    Material, Shape,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Debug)]
pub struct Object {
    transformation: Transformation,
    inverse_transformation: Transformation,
    pub material: Material,
    pub casts_shadow: bool,
    shape: Shape,
}

impl Object {
    #[must_use]
    #[allow(clippy::large_types_passed_by_value)]
    fn new(
        transformation: Transformation,
        material: Material,
        casts_shadow: bool,
        shape: Shape,
    ) -> Self {
        Self {
            transformation,
            inverse_transformation: transformation.invert(),
            material,
            casts_shadow,
            shape,
        }
    }
    #[must_use]
    pub fn new_plane(
        transformation: Transformation,
        material: Material,
        casts_shadow: bool,
    ) -> Self {
        Self::new(transformation, material, casts_shadow, Shape::new_plane())
    }

    #[must_use]
    pub fn default_plane() -> Self {
        Self::new_plane(Transformation::new(), Material::default(), true)
    }

    #[must_use]
    pub fn new_sphere(
        transformation: Transformation,
        material: Material,
        casts_shadow: bool,
    ) -> Self {
        Self::new(transformation, material, casts_shadow, Shape::new_sphere())
    }

    #[must_use]
    pub fn default_sphere() -> Self {
        Self::new_sphere(Transformation::new(), Material::default(), true)
    }

    #[must_use]
    pub fn new_glass_sphere(
        transformation: Transformation,
        casts_shadow: bool,
    ) -> Self {
        Self::new(
            transformation,
            Material {
                ambient: 0.01,
                diffuse: 0.01,
                transparency: 1.0,
                refractive_index: 1.5,
                ..Default::default()
            },
            casts_shadow,
            Shape::new_sphere(),
        )
    }

    #[must_use]
    pub fn default_glass_sphere() -> Self {
        Self::new_glass_sphere(Transformation::new(), true)
    }

    #[cfg(test)]
    #[must_use]
    pub fn new_test(
        transformation: Transformation,
        material: Material,
        casts_shadow: bool,
    ) -> Self {
        Self::new(transformation, material, casts_shadow, Shape::new_test())
    }

    #[cfg(test)]
    #[must_use]
    pub fn default_test() -> Self {
        Self::new_test(Transformation::new(), Material::default(), true)
    }

    #[must_use]
    pub fn to_object_space<'a, T: Transformable<'a>>(&self, value: &'a T) -> T {
        value.apply(&self.inverse_transformation)
    }

    #[must_use]
    pub fn to_world_space<'a, T: Transformable<'a>>(&self, value: &'a T) -> T {
        value.apply(&self.inverse_transformation.transpose())
    }
}

impl Intersectable for Object {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        let ray = self.to_object_space(ray);

        let Some(builder) = self.shape.intersect(&ray) else {
            return None;
        };

        Some(builder.object(self))
    }

    fn normal_at(&self, point: &Point) -> Vector {
        let object_point = self.to_object_space(point);

        let object_normal = self.shape.normal_at(&object_point);

        self.to_world_space(&object_normal).normalise()
    }
}

impl_approx_eq!(&Object { shape, transformation, ref material });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use paste::paste;

    use super::*;
    use crate::{
        math::{float::*, Angle},
        shape::Test,
        Colour,
    };

    #[test]
    fn creating_an_object() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let ti = t.invert();
        let m =
            Material { pattern: Colour::red().into(), ..Default::default() };

        /// Test the creation of objects using new_ and default_ functions.
        macro_rules! test_object {
            ($shape:ident) => {{
                paste! {
                    let s = Shape::[<new_ $shape:lower>]();

                    let o = Object::[<new_ $shape:lower>](t, m.clone(), false);

                    assert_approx_eq!(o.transformation, t);
                    assert_approx_eq!(o.inverse_transformation, ti);
                    assert_approx_eq!(o.material, &m);
                    assert!(!o.casts_shadow);
                    assert_approx_eq!(o.shape, s);

                    let o = Object::[<default_ $shape:lower>]();

                    assert_approx_eq!(o.transformation, Transformation::new());
                    assert_approx_eq!(
                        o.inverse_transformation, Transformation::new()
                    );
                    assert_approx_eq!(o.material, &Material::default());
                    assert!(o.casts_shadow);
                    assert_approx_eq!(o.shape, s);
                }
            }};
        }

        let s = Shape::new_plane();

        let o = Object::new(t, m.clone(), true, s);

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.inverse_transformation, ti);
        assert_approx_eq!(o.material, &m);
        assert!(o.casts_shadow);
        assert_approx_eq!(o.shape, s);

        test_object!(Plane);
        test_object!(Sphere);
        test_object!(Test);

        let m = Material {
            ambient: 0.01,
            diffuse: 0.01,
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        };
        let t = Transformation::new().shear(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let s = Shape::new_sphere();

        let o = Object::new_glass_sphere(t, false);

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.inverse_transformation, t.invert());
        assert_approx_eq!(o.material, &m);
        assert!(!o.casts_shadow);
        assert_approx_eq!(o.shape, s);

        let o = Object::default_glass_sphere();

        assert_approx_eq!(o.transformation, Transformation::new());
        assert_approx_eq!(o.inverse_transformation, Transformation::new());
        assert_approx_eq!(o.material, &m);
        assert_approx_eq!(o.shape, s);
    }

    #[test]
    fn intersecting_a_transformed_object_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_test(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
            true,
        );

        let i = o.intersect(&r);
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );

        let o = Object::new_test(
            Transformation::new().translate(5.0, 0.0, 0.0),
            Material::default(),
            true,
        );

        let i = o.intersect(&r);
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let o = Object::new_test(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material::default(),
            true,
        );

        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        let o = Object::new_test(
            Transformation::new()
                .rotate_z(Angle(PI / 5.0))
                .scale(1.0, 0.5, 1.0),
            Material::default(),
            true,
        );

        let sqrt_2_div_d = SQRT_2 / 2.0;
        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, sqrt_2_div_d, -sqrt_2_div_d)),
            Vector::new(0.0, 0.970_14, -0.242_54),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_sphere(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
            true,
        );

        let i = o.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap().build();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].object, &o);
        assert_approx_eq!(i[1].object, &o);

        assert_approx_eq!(i[0].t, 3.0);
        assert_approx_eq!(i[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_sphere(
            Transformation::new().translate(5.0, 0.0, 0.0),
            Material::default(),
            true,
        );

        let i = o.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let o = Object::new_sphere(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material::default(),
            true,
        );

        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let o = Object::new_sphere(
            Transformation::new()
                .rotate_z(Angle::from_degrees(36.0))
                .scale(1.0, 0.5, 1.0),
            Material::default(),
            true,
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, sqrt_2_div_2, -sqrt_2_div_2)),
            Vector::new(0.0, 0.970_14, -0.242_54),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn comparing_objects() {
        let o1 = Object::default_test();
        let o2 = Object::default_test();
        let o3 = Object::new_test(
            Transformation::new().scale(1.0, 2.0, 1.0),
            Material::default(),
            true,
        );

        assert_approx_eq!(o1, &o2);

        assert_approx_ne!(o1, &o3);
    }
}
