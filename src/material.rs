use derive_more::Constructor;

use crate::{
    math::{float::impl_approx_eq, Point, Vector},
    Colour, PointLight,
};

/// A `Material` represents what a given object is made up of including what
/// colour it is and how it reacts to light.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    #[must_use]
    pub fn lighting(
        &self,
        light: &PointLight,
        point: &Point,
        eye: &Vector,
        normal: &Vector,
    ) -> Colour {
        let colour = self.colour * light.intensity;

        let ambient = colour * self.ambient;

        let light_vector = (light.position - *point).normalise();
        let light_dot_normal = light_vector.dot(normal);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Colour::black(), Colour::black())
        } else {
            let diffuse = colour * self.diffuse * light_dot_normal;

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

impl_approx_eq!(Material { colour, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_material() {
        let m = Material::new(Colour::red(), 1.0, 1.0, 1.5, 25.6);

        assert_approx_eq!(m.colour, Colour::red());
        assert_approx_eq!(m.ambient, 1.0);
        assert_approx_eq!(m.diffuse, 1.0);
        assert_approx_eq!(m.specular, 1.5);
        assert_approx_eq!(m.shininess, 25.6);

        assert_approx_eq!(
            Material::default(),
            Material::new(Colour::white(), 0.1, 0.9, 0.9, 200.0)
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let p = Point::origin();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::white());

        assert_approx_eq!(
            m.lighting(&l, &p, &e, &n),
            Colour::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_eye_between_the_light_and_the_surface_at_45_degrees() {
        let m = Material::default();
        let p = Point::origin();

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        let e = Vector::new(0.0, sqrt_2_div_2, -sqrt_2_div_2);
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::white());

        assert_approx_eq!(m.lighting(&l, &p, &e, &n), Colour::white());
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let p = Point::origin();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::white());

        assert_approx_eq!(
            m.lighting(&l, &p, &e, &n),
            Colour::new(0.736_4, 0.736_4, 0.736_4),
            epsilon = 0.000_1
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_eye_in_the_path_of_reflection() {
        let m = Material::default();
        let p = Point::origin();

        let sqrt_2_div_2 = SQRT_2 / 2.0;
        let e = Vector::new(0.0, -sqrt_2_div_2, -sqrt_2_div_2);
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::white());

        assert_approx_eq!(
            m.lighting(&l, &p, &e, &n),
            Colour::new(1.636_4, 1.636_4, 1.636_4),
            epsilon = 0.000_1
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let p = Point::origin();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, 10.0), Colour::white());

        assert_approx_eq!(
            m.lighting(&l, &p, &e, &n),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn comparing_materials() {
        let m1 = Material::new(Colour::cyan(), 0.6, 0.3, 1.2, 142.7);
        let m2 = Material::new(Colour::cyan(), 0.6, 0.3, 1.2, 142.7);
        let m3 = Material::new(Colour::cyan(), 0.600_1, 0.3, 1.2, 142.7);

        assert_approx_eq!(m1, m2);

        assert_approx_ne!(m1, m3);
    }
}
