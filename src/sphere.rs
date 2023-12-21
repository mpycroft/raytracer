use derive_more::Constructor;

use crate::{
    intersect::{Intersectable, Intersection, IntersectionList},
    math::{
        float::impl_approx_eq, Point, Ray, Transformable, Transformation,
        Vector,
    },
    Material,
};

/// A `Sphere` is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Sphere {
    pub transformation: Transformation,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let ray = ray.apply(&self.transformation.invert());

        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();
        let a = 2.0 * a;

        let t1 = (-b - discriminant) / a;
        let t2 = (-b + discriminant) / a;

        Some(
            vec![Intersection::new(self, t1), Intersection::new(self, t2)]
                .into(),
        )
    }

    fn normal_at(&self, point: &Point) -> Vector {
        let inverse_transform = self.transformation.invert();

        let object_point = point.apply(&inverse_transform);

        let object_normal = object_point - Point::origin();

        let world_normal = object_normal.apply(&inverse_transform.transpose());

        world_normal.normalise()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Transformation::new(), Material::default())
    }
}

impl_approx_eq!(Sphere { transformation, material });

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI, SQRT_2};

    use super::*;
    use crate::{
        math::{float::*, Vector},
        Colour,
    };

    #[test]
    fn creating_a_sphere() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let m = Material { colour: Colour::red(), ..Default::default() };
        let s = Sphere::new(t, m);

        assert_approx_eq!(s.transformation, t);
        assert_approx_eq!(s.material, m);

        let s = Sphere::default();
        assert_approx_eq!(s.transformation, Transformation::new());
        assert_approx_eq!(s.material, Material::default());
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Sphere::default();

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].object, s);
        assert_approx_eq!(i[0].t, 4.0);
        assert_approx_eq!(i[1].object, s);
        assert_approx_eq!(i[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis());

        let s = Sphere::default();

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, 5.0);
        assert_approx_eq!(i[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis());

        let s = Sphere::default();

        let i = s.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::origin(), Vector::z_axis());

        let s = Sphere::default();

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, -1.0);
        assert_approx_eq!(i[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let s = Sphere::default();

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, -6.0);
        assert_approx_eq!(i[1].t, -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Sphere::new(
            Transformation::new().scale(2.0, 2.0, 2.0),
            Material::default(),
        );

        let i = s.intersect(&r);
        assert!(i.is_some());

        let i = i.unwrap();
        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, 3.0);
        assert_approx_eq!(i[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Sphere::new(
            Transformation::new().translate(5.0, 0.0, 0.0),
            Material::default(),
        );

        let i = s.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_an_axis() {
        let s = Sphere::default();

        assert_approx_eq!(
            s.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::x_axis()
        );

        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::y_axis()
        );

        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::z_axis()
        );
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = Sphere::default();

        let sqrt_3_div_3 = f64::sqrt(3.0) / 3.0;
        let n =
            s.normal_at(&Point::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));

        assert_approx_eq!(
            n,
            Vector::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3)
        );

        assert_approx_eq!(n, n.normalise());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material::default(),
        );

        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
            Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
        );
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let s = Sphere::new(
            Transformation::new().rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0),
            Material::default(),
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        assert_approx_eq!(
            s.normal_at(&Point::new(0.0, sqrt_2_div_2, -sqrt_2_div_2)),
            Vector::new(0.0, 0.970_14, -0.242_54),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn comparing_spheres() {
        let s1 = Sphere::new(
            Transformation::new().translate(0.5, 1.0, 0.0).scale(1.0, 2.0, 2.0),
            Material::default(),
        );
        let s2 = Sphere::new(
            Transformation::new().translate(0.5, 1.0, 0.0).scale(1.0, 2.0, 2.0),
            Material::default(),
        );
        let s3 = Sphere::new(
            Transformation::new()
                .translate(0.500_1, 1.0, 0.0)
                .scale(1.0, 2.0, 2.0),
            Material::default(),
        );

        assert_approx_eq!(s1, s2);

        assert_approx_ne!(s1, s3);
    }
}
