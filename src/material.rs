use derive_more::Constructor;

use crate::{
    math::{Point, Vector},
    Colour, PointLight,
};

/// Material represents what a given object is made up of including what colour
/// it is and how it reacts to light.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Point,
        eye: &Vector,
        normal: &Vector,
    ) -> Colour {
        let effective_colour = self.colour * light.intensity;

        let light_vector = (light.position - *point).normalise();

        let ambient = effective_colour * self.ambient;

        let light_dot_normal = light_vector.dot(normal);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colour::new(0.0, 0.0, 0.0), Colour::new(0.0, 0.0, 0.0))
        } else {
            let diffuse = effective_colour * self.diffuse * light_dot_normal;

            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);

            let specular = if reflect_dot_eye <= 0.0 {
                Colour::new(0.0, 0.0, 0.0)
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Colour::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}

add_approx_traits!(Material { colour, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use approx::*;

    use super::*;

    #[test]
    fn new() {
        let c = Colour::new(0.5, 0.3, 0.0);
        let m = Material::new(c, 0.5, 1.0, 0.6, 100.0);

        assert_relative_eq!(m.colour, c);
        assert_float_relative_eq!(m.ambient, 0.5);
        assert_float_relative_eq!(m.diffuse, 1.0);
        assert_float_relative_eq!(m.specular, 0.6);
        assert_float_relative_eq!(m.shininess, 100.0);
    }

    #[test]
    fn default() {
        let m = Material::default();

        assert_relative_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
        assert_float_relative_eq!(m.ambient, 0.1);
        assert_float_relative_eq!(m.diffuse, 0.9);
        assert_float_relative_eq!(m.specular, 0.9);
        assert_float_relative_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting() {
        let p = Point::origin();
        let m = Material::default();
        let c = Colour::new(1.0, 1.0, 1.0);

        let behind = PointLight::new(c, Point::new(0.0, 0.0, -10.0));
        let neg_z = Vector::new(0.0, 0.0, -1.0);

        assert_relative_eq!(
            m.lighting(&behind, &p, &neg_z, &neg_z,),
            Colour::new(1.9, 1.9, 1.9)
        );

        assert_relative_eq!(
            m.lighting(
                &behind,
                &p,
                &Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &neg_z
            ),
            Colour::new(1.0, 1.0, 1.0)
        );

        let above_left = PointLight::new(c, Point::new(0.0, 10.0, -10.0));

        assert_relative_eq!(
            m.lighting(&above_left, &p, &neg_z, &neg_z),
            Colour::new(0.736_396, 0.736_396, 0.736_396)
        );

        assert_relative_eq!(
            m.lighting(
                &above_left,
                &p,
                &Vector::new(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &neg_z
            ),
            Colour::new(1.636_396, 1.636_396, 1.636_396)
        );

        assert_relative_eq!(
            m.lighting(
                &PointLight::new(c, Point::new(0.0, 0.0, 10.0)),
                &p,
                &neg_z,
                &neg_z
            ),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn approx() {
        let m1 =
            Material::new(Colour::new(0.3, 0.4, 1.0), 0.2, 0.4, 0.3, 150.0);
        let m2 =
            Material::new(Colour::new(0.3, 0.4, 1.0), 0.2, 0.4, 0.3, 150.0);
        let m3 = Material::new(
            Colour::new(0.3, 0.4, 1.000_1),
            0.2,
            0.400_09,
            0.3,
            150.01,
        );

        assert_abs_diff_eq!(m1, m2);
        assert_abs_diff_ne!(m1, m3);

        assert_relative_eq!(m1, m2);
        assert_relative_ne!(m1, m3);

        assert_ulps_eq!(m1, m2);
        assert_ulps_ne!(m1, m3);
    }
}
