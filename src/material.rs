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
        in_shadow: bool,
    ) -> Colour {
        let effective_colour = self.colour * light.intensity;

        let ambient = effective_colour * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_vector = (light.position - *point).normalise();

        let light_dot_normal = light_vector.dot(normal);
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colour::black(), Colour::black())
        } else {
            let diffuse = effective_colour * self.diffuse * light_dot_normal;

            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);

            let specular = if reflect_dot_eye <= 0.0 {
                Colour::black()
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
        Self::new(Colour::white(), 0.1, 0.9, 0.9, 200.0)
    }
}

add_approx_traits!(Material { colour, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use approx::*;

    use super::*;

    #[test]
    fn creating_a_new_material() {
        let c = Colour::new(0.5, 0.3, 0.0);
        let m = Material::new(c, 0.5, 1.0, 0.6, 100.0);

        assert_relative_eq!(m.colour, c);
        assert_float_relative_eq!(m.ambient, 0.5);
        assert_float_relative_eq!(m.diffuse, 1.0);
        assert_float_relative_eq!(m.specular, 0.6);
        assert_float_relative_eq!(m.shininess, 100.0);
    }

    #[test]
    fn the_default_material() {
        let m = Material::default();

        assert_relative_eq!(m.colour, Colour::white());
        assert_float_relative_eq!(m.ambient, 0.1);
        assert_float_relative_eq!(m.diffuse, 0.9);
        assert_float_relative_eq!(m.specular, 0.9);
        assert_float_relative_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface() {
        let neg_z = -Vector::z_axis();

        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 0.0, -10.0)),
                &Point::origin(),
                &neg_z,
                &neg_z,
                false
            ),
            Colour::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 0.0, -10.0)),
                &Point::origin(),
                &Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &-Vector::z_axis(),
                false
            ),
            Colour::white()
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let neg_z = -Vector::z_axis();

        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 10.0, -10.0)),
                &Point::origin(),
                &neg_z,
                &neg_z,
                false
            ),
            Colour::new(0.736_396, 0.736_396, 0.736_396)
        );
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 10.0, -10.0)),
                &Point::origin(),
                &Vector::new(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                &-Vector::z_axis(),
                false
            ),
            Colour::new(1.636_396, 1.636_396, 1.636_396)
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let neg_z = -Vector::z_axis();

        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 0.0, 10.0)),
                &Point::origin(),
                &neg_z,
                &neg_z,
                false
            ),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let neg_z = -Vector::z_axis();

        assert_relative_eq!(
            Material::default().lighting(
                &PointLight::new(Colour::white(), Point::new(0.0, 0.0, -10.0)),
                &Point::origin(),
                &neg_z,
                &neg_z,
                true
            ),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn materials_are_approximately_equal() {
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
