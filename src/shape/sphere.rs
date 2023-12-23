use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{Point, Ray, Vector},
};

/// A `Sphere` is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug)]
pub struct Sphere;

impl Intersectable for Sphere {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
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

        Some(ListBuilder::new().add_t(t1).add_t(t2))
    }

    fn normal_at(&self, point: &Point) -> Vector {
        *point - Point::origin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::*, Vector},
        Object,
    };

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let b = s.intersect(&r);

        assert!(b.is_some());

        let o = Object::default_sphere();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, 4.0);
        assert_approx_eq!(i[1].t, 6.0);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let b = s.intersect(&r);

        assert!(b.is_some());

        let o = Object::default_sphere();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, 5.0);
        assert_approx_eq!(i[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::z_axis());

        let s = Sphere;

        let i = s.intersect(&r);
        assert!(i.is_none());
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::origin(), Vector::z_axis());

        let s = Sphere;

        let b = s.intersect(&r);

        assert!(b.is_some());

        let o = Object::default_sphere();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, -1.0);
        assert_approx_eq!(i[1].t, 1.0);
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let s = Sphere;

        let b = s.intersect(&r);

        assert!(b.is_some());

        let o = Object::default_sphere();
        let i = b.unwrap().object(&o).build();

        assert_eq!(i.len(), 2);

        assert_approx_eq!(i[0].t, -6.0);
        assert_approx_eq!(i[1].t, -4.0);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_an_axis() {
        let s = Sphere;

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
        let s = Sphere;
        let sqrt_3_div_3 = f64::sqrt(3.0) / 3.0;
        let n =
            s.normal_at(&Point::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3));

        assert_approx_eq!(
            n,
            Vector::new(sqrt_3_div_3, sqrt_3_div_3, sqrt_3_div_3)
        );

        assert_approx_eq!(n, n.normalise());
    }
}
