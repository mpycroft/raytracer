use derive_more::Constructor;

use crate::{
    math::{Point, Vector},
    util::float::Float,
    Colour, Object, Pattern, PointLight,
};

/// Material represents what a given object is made up of including what colour
/// it is and how it reacts to light.
#[derive(Clone, Debug, PartialEq, PartialOrd, Constructor)]
pub struct Material<T: Float> {
    pub pattern: Pattern<T>,
    pub ambient: T,
    pub diffuse: T,
    pub specular: T,
    pub shininess: T,
}

impl<T: Float> Material<T> {
    pub fn lighting(
        &self,
        object: &Object<T>,
        light: &PointLight<T>,
        point: &Point<T>,
        eye: &Vector<T>,
        normal: &Vector<T>,
        in_shadow: bool,
    ) -> Colour<T> {
        let colour = self.pattern.pattern_at(object, point);

        let effective_colour = colour * light.intensity;

        let ambient = effective_colour * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_vector = (light.position - *point).normalise();

        let light_dot_normal = light_vector.dot(normal);
        let (diffuse, specular) = if light_dot_normal < T::zero() {
            (Colour::black(), Colour::black())
        } else {
            let diffuse = effective_colour * self.diffuse * light_dot_normal;

            let reflect_vector = -light_vector.reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);

            let specular = if reflect_dot_eye <= T::zero() {
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

impl<T: Float> Default for Material<T> {
    fn default() -> Self {
        Self::new(
            Pattern::default_uniform(Colour::white()),
            T::from(0.1f64).unwrap(),
            T::from(0.9f64).unwrap(),
            T::from(0.9f64).unwrap(),
            T::from(200.0f64).unwrap(),
        )
    }
}

add_approx_traits!(Material<T> { pattern, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use approx::*;

    use super::*;

    #[test]
    fn creating_a_new_material() {
        let p = Pattern::default_uniform(Colour::new(0.5, 0.3, 0.0));
        let m = Material::new(p.clone(), 0.5, 1.0, 0.6, 100.0);

        assert_relative_eq!(m.pattern, p);
        assert_float_relative_eq!(m.ambient, 0.5);
        assert_float_relative_eq!(m.diffuse, 1.0);
        assert_float_relative_eq!(m.specular, 0.6);
        assert_float_relative_eq!(m.shininess, 100.0);
    }

    #[test]
    fn the_default_material() {
        let m = Material::<f64>::default();

        assert_relative_eq!(
            m.pattern,
            Pattern::default_uniform(Colour::white())
        );
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
                &Object::default_sphere(),
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
                &Object::default_sphere(),
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
                &Object::default_sphere(),
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
                &Object::default_sphere(),
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
                &Object::default_sphere(),
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
                &Object::default_sphere(),
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
    fn lighting_with_a_pattern_applied() {
        let o = Object::default_sphere();
        let m = Material::new(
            Pattern::default_stripe(Colour::white(), Colour::black()),
            1.0,
            0.0,
            0.0,
            0.0,
        );

        let l = PointLight::new(Colour::white(), Point::new(0.0, 0.0, -10.0));
        let neg_z = -Vector::z_axis();

        assert_relative_eq!(
            m.lighting(
                &o,
                &l,
                &Point::new(0.9, 0.0, 0.0),
                &neg_z,
                &neg_z,
                false
            ),
            Colour::white()
        );
        assert_relative_eq!(
            m.lighting(
                &o,
                &l,
                &Point::new(1.1, 0.0, 0.0),
                &neg_z,
                &neg_z,
                false
            ),
            Colour::black()
        );
    }

    #[test]
    fn materials_are_approximately_equal() {
        let m1 = Material::new(
            Pattern::default_uniform(Colour::new(0.3, 0.4, 1.0)),
            0.2,
            0.4,
            0.3,
            150.0,
        );
        let m2 = Material::new(
            Pattern::default_uniform(Colour::new(0.3, 0.4, 1.0)),
            0.2,
            0.4,
            0.3,
            150.0,
        );
        let m3 = Material::new(
            Pattern::default_uniform(Colour::new(0.3, 0.4, 1.000_1)),
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
