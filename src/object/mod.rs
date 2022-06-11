mod plane;
mod sphere;
#[cfg(test)]
mod test;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use paste::paste;

#[cfg(test)]
use self::test::Test;
use self::{plane::Plane, sphere::Sphere};
use crate::{
    intersect::{Intersectable, IntersectionPoints},
    math::{Point, Ray, Transform, Vector},
    util::{
        approx::{FLOAT_EPSILON, FLOAT_ULPS},
        float::Float,
    },
    Material,
};

/// An `Object` represents some entity in the scene that can be rendered.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Object<T: Float> {
    pub transform: Transform<T>,
    pub material: Material<T>,
    pub shape: Shape<T>,
}

macro_rules! add_shape_fns {
    (@add $fn_new:ident, $fn_default:ident, $shape:ident) => {
        pub fn $fn_new(transform: Transform<T>, material: Material<T>) -> Self {
            Self::new(transform, material, Shape::$shape($shape::new()))
        }

        pub fn $fn_default() -> Self {
            Self::default(Shape::$shape($shape::default()))
        }
    };
    ($($shape:ident),+) => {
        $(
            paste! {
                add_shape_fns!(
                    @add [<new_ $shape:snake>],
                    [<default_ $shape:snake>],
                    $shape
                );
            }
        )+
    };
}

impl<T: Float> Object<T> {
    fn new(
        transform: Transform<T>,
        material: Material<T>,
        shape: Shape<T>,
    ) -> Self {
        Self { transform, material, shape }
    }

    fn default(shape: Shape<T>) -> Self {
        Self::new(Transform::default(), Material::default(), shape)
    }

    pub fn new_glass_sphere(
        transform: Transform<T>,
        refractive_index: T,
    ) -> Self {
        Self::new(
            transform,
            Material {
                transparency: T::one(),
                refractive_index,
                ..Default::default()
            },
            Shape::Sphere(Sphere::new()),
        )
    }

    pub fn default_glass_sphere() -> Self {
        Self::new_glass_sphere(Transform::default(), T::convert(1.5))
    }

    add_shape_fns!(Sphere, Plane);

    #[cfg(test)]
    add_shape_fns!(Test);
}

impl<T: Float> Intersectable<T> for Object<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>> {
        let local_ray = self.transform.invert().apply(ray);

        self.shape.intersect(&local_ray)
    }

    fn normal_at(&self, point: &Point<T>) -> Vector<T> {
        let local_point = self.transform.invert().apply(point);

        let local_normal = self.shape.normal_at(&local_point);

        self.transform.invert().transpose().apply(&local_normal).normalise()
    }
}

add_approx_traits!(Object<T> { transform, material, shape });

/// Shape is a list of the various geometries that can be rendered.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Shape<T: Float> {
    Sphere(Sphere<T>),
    Plane(Plane<T>),

    #[cfg(test)]
    Test(Test<T>),
}

impl<T: Float> Intersectable<T> for Shape<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionPoints<T>> {
        match self {
            Shape::Sphere(sphere) => sphere.intersect(ray),
            Shape::Plane(plane) => plane.intersect(ray),

            #[cfg(test)]
            Shape::Test(test) => test.intersect(ray),
        }
    }

    fn normal_at(&self, point: &Point<T>) -> Vector<T> {
        match self {
            Shape::Sphere(sphere) => sphere.normal_at(point),
            Shape::Plane(plane) => plane.normal_at(point),

            #[cfg(test)]
            Shape::Test(test) => test.normal_at(point),
        }
    }
}

impl<T> AbsDiffEq for Shape<T>
where
    T: Float + AbsDiffEq,
    T::Epsilon: Float,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::Epsilon::convert(FLOAT_EPSILON)
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        match (self, other) {
            (Shape::Sphere(lhs), Shape::Sphere(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }
            (Shape::Plane(lhs), Shape::Plane(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }

            #[cfg(test)]
            (Shape::Test(lhs), Shape::Test(rhs)) => {
                lhs.abs_diff_eq(rhs, epsilon)
            }

            (_, _) => false,
        }
    }
}

impl<T> RelativeEq for Shape<T>
where
    T: Float + RelativeEq,
    T::Epsilon: Float,
{
    fn default_max_relative() -> Self::Epsilon {
        T::Epsilon::convert(FLOAT_EPSILON)
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        match (self, other) {
            (Shape::Sphere(lhs), Shape::Sphere(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }
            (Shape::Plane(lhs), Shape::Plane(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }

            #[cfg(test)]
            (Shape::Test(lhs), Shape::Test(rhs)) => {
                lhs.relative_eq(rhs, epsilon, max_relative)
            }

            (_, _) => false,
        }
    }
}

impl<T> UlpsEq for Shape<T>
where
    T: Float + UlpsEq,
    T::Epsilon: Float,
{
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
            (Shape::Sphere(lhs), Shape::Sphere(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }
            (Shape::Plane(lhs), Shape::Plane(rhs)) => {
                lhs.ulps_eq(rhs, epsilon, max_ulps)
            }

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
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use approx::*;

    use super::*;
    use crate::math::Angle;

    #[test]
    fn creating_a_new_object() {
        let t = Transform::from_scale(1.0, 2.0, 2.0);
        let m = Material::default();
        let s = Shape::Test(Test::default());

        let o = Object::new(t, m.clone(), s.clone());

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, s);
    }

    #[test]
    fn an_objects_default_transformation_and_material() {
        let s = Shape::<f64>::Test(Test::default());

        let o = Object::default(s.clone());

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, s);
    }

    #[test]
    fn creating_a_new_glass_sphere() {
        let t = Transform::from_translate(1.0, 1.0, 2.0);
        let o = Object::new_glass_sphere(t, 1.2);

        let m = Material {
            transparency: 1.0,
            refractive_index: 1.2,
            ..Default::default()
        };

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Sphere(Sphere::new()));
    }

    #[test]
    fn creating_a_default_glass_sphere() {
        let o = Object::default_glass_sphere();

        let m = Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        };

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Sphere(Sphere::default()));
    }

    #[test]
    fn creating_a_new_sphere() {
        let t = Transform::<f64>::from_shear(0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
        let m = Material::default();

        let o = Object::new_sphere(t, m.clone());

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Sphere(Sphere::new()));
    }

    #[test]
    fn creating_a_default_sphere() {
        let o = Object::<f64>::default_sphere();

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, Shape::Sphere(Sphere::default()));
    }

    #[test]
    fn creating_a_new_plane() {
        let t = Transform::from_rotate_x(Angle::from_degrees(30.0f64));
        let m = Material::default();

        let o = Object::new_plane(t, m.clone());

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Plane(Plane::new()));
    }

    #[test]
    fn creating_a_default_plane() {
        let o = Object::<f64>::default_plane();

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, Shape::Plane(Plane::default()));
    }

    #[test]
    fn creating_a_new_test_object() {
        let t = Transform::from_scale(1.0, 0.5, 1.0);
        let m = Material::default();

        let o = Object::new_test(t, m.clone());

        assert_relative_eq!(o.transform, t);
        assert_relative_eq!(o.material, m);
        assert_relative_eq!(o.shape, Shape::Test(Test::new()));
    }

    #[test]
    fn creating_a_default_test_object() {
        let o = Object::<f64>::default_test();

        assert_relative_eq!(o.transform, Transform::default());
        assert_relative_eq!(o.material, Material::default());
        assert_relative_eq!(o.shape, Shape::Test(Test::default()));
    }

    #[test]
    fn intersecting_a_scaled_object_with_a_ray() {
        let o = Object::new_test(
            Transform::from_scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        o.intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()));

        let test = match o.shape {
            Shape::Test(test) => test,
            _ => unreachable!(),
        };

        assert_relative_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(0.0, 0.0, -2.5), Vector::new(0.0, 0.0, 0.5))
        );
    }

    #[test]
    fn intersecting_a_translated_object_with_a_ray() {
        let o = Object::new_test(
            Transform::from_translate(5.0, 0.0, 0.0),
            Material::default(),
        );

        o.intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()));

        let test = match o.shape {
            Shape::Test(test) => test,
            _ => unreachable!(),
        };

        assert_relative_eq!(
            test.ray.get().unwrap(),
            Ray::new(Point::new(-5.0, 0.0, -5.0), Vector::z_axis())
        );
    }

    #[test]
    fn computing_the_normal_on_a_translated_object() {
        assert_relative_eq!(
            Object::new_test(
                Transform::from_translate(0.0, 1.0, 0.0),
                Material::default(),
            )
            .normal_at(&Point::new(
                0.0,
                FRAC_1_SQRT_2 + 1.0,
                -FRAC_1_SQRT_2
            )),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_object() {
        assert_relative_eq!(
            Object::new_test(
                Transform::from_rotate_z(Angle::from_radians(PI / 5.0))
                    .scale(1.0, 0.5, 1.0),
                Material::default()
            )
            .normal_at(&Point::new(
                0.0,
                SQRT_2 / 2.0,
                -SQRT_2 / 2.0
            )),
            Vector::new(0.0, 0.970_143, -0.242_536)
        );
    }

    #[test]
    fn objects_are_approximately_equal() {
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
        let o3 = Object::default_sphere();

        assert_abs_diff_eq!(o1, o2);
        assert_abs_diff_ne!(o1, o3);

        assert_relative_eq!(o1, o2);
        assert_relative_ne!(o1, o3);

        assert_ulps_eq!(o1, o2);
        assert_ulps_ne!(o1, o3);
    }
}
