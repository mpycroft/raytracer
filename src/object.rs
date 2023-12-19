use derive_more::Constructor;

use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{
        float::impl_approx_eq, Point, Ray, Transformable, Transformation,
        Vector,
    },
    Material, Shape,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Object {
    pub transformation: Transformation,
    pub material: Material,
    pub shape: Shape,
}

impl Object {
    #[must_use]
    pub fn new_sphere(
        transformation: Transformation,
        material: Material,
    ) -> Self {
        Self::new(transformation, material, Shape::new_sphere())
    }

    #[must_use]
    pub fn default_sphere() -> Self {
        Self::new(
            Transformation::new(),
            Material::default(),
            Shape::new_sphere(),
        )
    }

    #[cfg(test)]
    #[must_use]
    pub fn new_test(
        transformation: Transformation,
        material: Material,
    ) -> Self {
        Self::new(transformation, material, Shape::new_test())
    }

    #[cfg(test)]
    #[must_use]
    pub fn default_test() -> Self {
        Self::new(Transformation::new(), Material::default(), Shape::new_test())
    }
}

impl Intersectable for Object {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        let ray = ray.apply(&self.transformation.invert());

        let Some(builder) = self.shape.intersect(&ray) else {
            return None;
        };

        Some(builder.object(self))
    }

    fn normal_at(&self, point: &Point) -> Vector {
        let inverse_transform = self.transformation.invert();

        let object_point = point.apply(&inverse_transform);

        let object_normal = self.shape.normal_at(&object_point);

        let world_normal = object_normal.apply(&inverse_transform.transpose());

        world_normal.normalise()
    }
}

impl_approx_eq!(Object { shape, transformation, material });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use super::*;
    use crate::{
        math::{float::*, Angle},
        shape::Test,
        Colour,
    };

    #[test]
    fn creating_an_object() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let m = Material { colour: Colour::red(), ..Default::default() };
        let s = Shape::new_sphere();

        let o = Object::new_sphere(t, m);

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.material, m);
        assert_approx_eq!(o.shape, s);

        let o = Object::default_sphere();

        assert_approx_eq!(o.transformation, Transformation::new());
        assert_approx_eq!(o.material, Material::default());
        assert_approx_eq!(o.shape, s);

        let s = Shape::new_test();

        let o = Object::new_test(t, m);

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.material, m);
        assert_approx_eq!(o.shape, s);

        let o = Object::default_test();

        assert_approx_eq!(o.transformation, Transformation::new());
        assert_approx_eq!(o.material, Material::default());
        assert_approx_eq!(o.shape, s);
    }

    #[test]
    fn intersecting_a_transformed_object_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let mut o = Object::new_test(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        let i = o.intersect(&r);
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );

        o.transformation = Transformation::new().translate(5.0, 0.0, 0.0);

        let i = o.intersect(&r);
        let l = i.unwrap().object(&o).build();

        assert_approx_eq!(
            Test::intersection_to_ray(&l),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut o = Object::new_test(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material::default(),
        );

        assert_approx_eq!(
            o.normal_at(&Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );

        o.transformation = Transformation::new()
            .rotate_z(Angle(PI / 5.0))
            .scale(1.0, 0.5, 1.0);

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
        );

        let i = o.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap().build();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].object, o);
        assert_approx_eq!(i[1].object, o);

        assert_approx_eq!(i[0].t, 3.0);
        assert_approx_eq!(i[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_sphere(
            Transformation::new().translate(5.0, 0.0, 0.0),
            Material::default(),
        );

        let i = o.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let o = Object::new_sphere(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material::default(),
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
        );

        assert_approx_eq!(o1, o2);

        assert_approx_ne!(o1, o3);
    }
}
