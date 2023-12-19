use derive_more::Constructor;
use float_cmp::{ApproxEq, F64Margin};

use crate::{
    intersect::{Intersectable, IntersectionList},
    math::{Point, Ray, Transformable, Transformation, Vector},
    Material, Shape,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Debug, Constructor)]
pub struct Object {
    pub transformation: Transformation,
    pub material: Material,
    pub shape: Shape,
}

impl Object {
    #[must_use]
    pub fn new_test() -> Self {
        Self::new(Transformation::new(), Material::default(), Shape::new_test())
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let ray = ray.apply(&self.transformation.invert());

        self.shape.intersect(&ray)
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

impl ApproxEq for &Object {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.shape.approx_eq(&other.shape, margin)
            && self.transformation.approx_eq(other.transformation, margin)
            && self.material.approx_eq(other.material, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Colour};

    #[test]
    fn creating_an_object() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let m = Material { colour: Colour::red(), ..Default::default() };
        let s = Shape::new_test();
        let o = Object::new(t, m, s.clone());

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.material, m);
        assert_approx_eq!(o.shape, &s);

        let o = Object::new_test();
        assert_approx_eq!(o.transformation, Transformation::new());
        assert_approx_eq!(o.material, Material::default());
        assert_approx_eq!(o.shape, &s);
    }

    #[test]
    fn intersecting_a_transformed_object_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let mut o = Object::new(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
            Shape::new_test(),
        );

        let _ = o.intersect(&r);

        let Shape::Test(test) = o.shape.clone();
        assert_approx_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );

        o.transformation = Transformation::new().translate(5.0, 0.0, 0.0);

        let _ = o.intersect(&r);

        let Shape::Test(test) = o.shape;
        assert_approx_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn comparing_objects() {
        let o1 = Object::new_test();
        let o2 = Object::new_test();
        let o3 = Object::new(
            Transformation::new().scale(1.0, 2.0, 1.0),
            Material::default(),
            Shape::new_test(),
        );

        assert_approx_eq!(o1, &o2);

        assert_approx_ne!(o1, &o3);
    }
}
