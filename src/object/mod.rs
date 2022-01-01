#[cfg(test)]
mod test;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(test)]
use self::test::Test;
use crate::{
    intersect::{Intersectable, IntersectionList},
    math::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        Point, Ray, Transform, Vector,
    },
    Material,
};

/// An Object represents some entity in the scene that can be rendered.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Object {
    transform: Transform,
    material: Material,
    shape: Shape,
}

impl Object {
    fn new(transform: Transform, material: Material, shape: Shape) -> Self {
        Self { transform, material, shape }
    }

    fn default(shape: Shape) -> Self {
        Self::new(Transform::default(), Material::default(), shape)
    }

    #[cfg(test)]
    pub fn new_test(transform: Transform, material: Material) -> Self {
        Self::new(transform, material, Shape::Test(Test::default()))
    }

    #[cfg(test)]
    pub fn default_test() -> Self {
        Self::default(Shape::Test(Test::default()))
    }
}

impl Intersectable for Object {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let local_ray = self.transform.invert().apply(ray);

        self.shape.intersect(&local_ray)
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

add_approx_traits!(Object { transform, material, shape });

/// Shape is a list of the various geometries that can be rendered.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Shape {
    #[cfg(test)]
    Test(Test),
}

impl Intersectable for Shape {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        match self {
            #[cfg(test)]
            Shape::Test(test) => test.intersect(ray),
            _ => todo!(),
        }
    }

    fn normal_at(&self, point: &Point) -> Vector {
        todo!()
    }
}

impl AbsDiffEq for Shape {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self, other) {
            #[cfg(test)]
            (Shape::Test(lhs), Shape::Test(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
            (_, _) => false,
        }
    }
}

impl RelativeEq for Shape {
    fn default_max_relative() -> Self::Epsilon {
        FLOAT_EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self, other) {
            #[cfg(test)]
            (Shape::Test(lhs), Shape::Test(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
            (_, _) => false,
        }
    }
}

impl UlpsEq for Shape {
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_ulps: u32,
    ) -> bool {
        match (self, other) {
            #[cfg(test)]
            (Shape::Test(lhs), Shape::Test(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn new() {
        let t = Transform::from_scale(1.0, 2.0, 2.0);
        let m = Material::default();
        let s = Shape::Test(Test::default());

        let o = Object::new(t, m, s.clone());

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, s);
    }

    #[test]
    fn default() {
        let s = Shape::Test(Test::default());

        let o = Object::default(s.clone());

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, s);
    }

    #[test]
    fn new_test() {
        let t = Transform::from_scale(1.0, 0.5, 1.0);
        let m = Material::default();

        let o = Object::new_test(t, m);

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Test(Test::default()));
    }

    #[test]
    fn default_test() {
        let o = Object::default_test();

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, Shape::Test(Test::default()));
    }

    #[test]
    fn intersect() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let o = Object::new_test(
            Transform::from_scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        o.intersect(&r);

        let Shape::Test(test) = o.shape;
        assert_relative_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );

        let o = Object::new_test(
            Transform::from_translate(5.0, 0.0, 0.0),
            Material::default(),
        );

        o.intersect(&r);

        let Shape::Test(test) = o.shape;
        assert_relative_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn approx() {
        let o1 = Object::new(
            Transform::from_translate(5.0, 4.0, 3.0),
            Material::default(),
            Shape::Test(Test::default()),
        );
        let o2 = Object::new(
            Transform::from_translate(5.0, 4.0, 3.0),
            Material::default(),
            Shape::Test(Test::default()),
        );
        let o3 = Object::default_test();

        assert_abs_diff_eq!(o1, o2);
        assert_abs_diff_ne!(o1, o3);

        assert_relative_eq!(o1, o2);
        assert_relative_ne!(o1, o3);

        assert_ulps_eq!(o1, o2);
        assert_ulps_ne!(o1, o3);
    }
}
