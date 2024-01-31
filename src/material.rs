use typed_builder::TypedBuilder;

use crate::{
    math::{float::impl_approx_eq, Point, Vector},
    Colour, Pattern, PointLight, Shape,
};

/// A `Material` represents what a given object is made up of including what
/// colour it is and how it reacts to light.
#[derive(Clone, Debug, TypedBuilder)]
#[allow(clippy::too_many_arguments)]
pub struct Material {
    #[builder(default = Colour::white().into())]
    pub pattern: Pattern,
    #[builder(default = 0.1)]
    pub ambient: f64,
    #[builder(default = 0.9)]
    pub diffuse: f64,
    #[builder(default = 0.9)]
    pub specular: f64,
    #[builder(default = 200.0)]
    pub shininess: f64,
    #[builder(default = 0.0)]
    pub reflective: f64,
    #[builder(default = 0.0)]
    pub transparency: f64,
    #[builder(default = 1.0)]
    pub refractive_index: f64,
}

impl Material {
    #[must_use]
    pub fn glass() -> Self {
        Self::builder()
            .ambient(0.01)
            .diffuse(0.01)
            .transparency(1.0)
            .refractive_index(1.5)
            .build()
    }

    #[must_use]
    pub fn lighting(
        &self,
        object: &Shape,
        light: &PointLight,
        point: &Point,
        eye: &Vector,
        normal: &Vector,
        in_shadow: bool,
    ) -> Colour {
        let colour = self.pattern.pattern_at(object, point) * light.intensity;

        let ambient = colour * self.ambient;

        if in_shadow {
            return ambient;
        }

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
        Self::builder().build()
    }
}

impl_approx_eq!(
    &Material { ref pattern, ambient, diffuse, specular, shininess, reflective }
);

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::{math::float::*, pattern::Pattern, Object};

    #[test]
    fn creating_a_material() {
        let m = Material::builder()
            .pattern(Colour::red().into())
            .ambient(1.0)
            .diffuse(1.0)
            .specular(1.5)
            .shininess(25.6)
            .reflective(0.6)
            .transparency(0.5)
            .refractive_index(1.5)
            .build();

        assert_approx_eq!(
            m.pattern,
            &Pattern::solid_builder(Colour::red()).build()
        );
        assert_approx_eq!(m.ambient, 1.0);
        assert_approx_eq!(m.diffuse, 1.0);
        assert_approx_eq!(m.specular, 1.5);
        assert_approx_eq!(m.shininess, 25.6);
        assert_approx_eq!(m.reflective, 0.6);
        assert_approx_eq!(m.transparency, 0.5);
        assert_approx_eq!(m.refractive_index, 1.5);

        assert_approx_eq!(
            Material::default(),
            &Material {
                pattern: Colour::white().into(),
                ambient: 0.1,
                diffuse: 0.9,
                specular: 0.9,
                shininess: 200.0,
                reflective: 0.0,
                transparency: 0.0,
                refractive_index: 1.0
            }
        );

        assert_approx_eq!(
            Material::glass(),
            &Material {
                pattern: Colour::white().into(),
                ambient: 0.01,
                diffuse: 0.01,
                specular: 0.9,
                shininess: 200.0,
                reflective: 0.0,
                transparency: 1.0,
                refractive_index: 1.5
            }
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let p = Point::origin();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, true),
            Colour::new(0.1, 0.1, 0.1)
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
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
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
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
            Colour::white()
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_the_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::default();
        let p = Point::origin();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 10.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
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
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
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
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_reflect_dot_eye_less_than_zero() {
        let m = Material::default();
        let p = Point::origin();

        let e = Vector::new(0.0, 0.0, 0.3);
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, false),
            Colour::white()
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_with_a_pattern_applied() {
        let m = Material::builder()
            .pattern(
                Pattern::stripe_builder(
                    Colour::white().into(),
                    Colour::black().into(),
                )
                .build(),
            )
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .build();

        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        let l = PointLight::new(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &Point::new(0.9, 0.0, 0.0), &e, &n, false),
            Colour::white()
        );

        assert_approx_eq!(
            m.lighting(&o, &l, &Point::new(1.1, 0.0, 0.0), &e, &n, false),
            Colour::black()
        );
    }

    #[test]
    fn comparing_materials() {
        let m1 = Material::builder()
            .pattern(Colour::cyan().into())
            .ambient(0.6)
            .diffuse(0.3)
            .specular(1.2)
            .shininess(142.7)
            .reflective(0.3)
            .transparency(0.1)
            .refractive_index(1.1)
            .build();
        let m2 = Material::builder()
            .pattern(Colour::cyan().into())
            .ambient(0.6)
            .diffuse(0.3)
            .specular(1.2)
            .shininess(142.7)
            .reflective(0.3)
            .transparency(0.1)
            .refractive_index(1.1)
            .build();
        let m3 = Material::builder()
            .pattern(Colour::cyan().into())
            .ambient(0.600_1)
            .diffuse(0.3)
            .specular(1.2)
            .shininess(142.7)
            .reflective(0.3)
            .transparency(0.1)
            .refractive_index(1.1)
            .build();

        assert_approx_eq!(m1, &m2);

        assert_approx_ne!(m1, &m3);
    }
}
