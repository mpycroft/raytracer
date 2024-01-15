use derive_new::new;

use crate::{
    math::{Point, Vector},
    Object,
};

/// The `Computations` struct is a helper structure to store precomputed values
/// about an intersection.
#[derive(Clone, Copy, Debug, new)]
#[allow(clippy::too_many_arguments)]
pub struct Computations<'a> {
    pub object: &'a Object,
    pub t: f64,
    pub point: Point,
    pub over_point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub reflect: Vector,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Point,
}

impl<'a> Computations<'a> {
    #[must_use]
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye.dot(&self.normal);

        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::{
        intersection::List,
        math::{float::*, Ray},
        Intersection, Material,
    };

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let o = Object::sphere_builder().material(Material::glass()).build();

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(Point::new(0.0, 0.0, sqrt_2_div_2), Vector::y_axis());

        let l = List::from(vec![
            Intersection::new(&o, -sqrt_2_div_2),
            Intersection::new(&o, sqrt_2_div_2),
        ]);

        let c = l[1].prepare_computations(&r, &l);

        assert_approx_eq!(c.schlick(), 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let o = Object::sphere_builder().material(Material::glass()).build();

        let r = Ray::new(Point::origin(), Vector::y_axis());

        let l = List::from(vec![
            Intersection::new(&o, -1.0),
            Intersection::new(&o, 1.0),
        ]);

        let c = l[1].prepare_computations(&r, &l);

        assert_approx_eq!(c.schlick(), 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_n1() {
        let o = Object::sphere_builder().material(Material::glass()).build();

        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::z_axis());

        let l = List::from(Intersection::new(&o, 1.858_9));

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(c.schlick(), 0.488_73, epsilon = 0.000_01);
    }
}
