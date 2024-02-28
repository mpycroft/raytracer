use rand::prelude::*;
use serde::{de::Error, Deserialize, Deserializer};
use typed_builder::TypedBuilder;

use crate::{
    light::Lightable,
    math::{float::impl_approx_eq, Point, Vector},
    Colour, Light, Object, Pattern,
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
    #[allow(clippy::too_many_arguments)]
    pub fn lighting<R: Rng>(
        &self,
        object: &Object,
        light: &Light,
        point: &Point,
        eye: &Vector,
        normal: &Vector,
        intensity: f64,
        rng: &mut R,
    ) -> Colour {
        let colour = self.pattern.pattern_at(object, point) * light.intensity();

        let ambient = colour * self.ambient;

        let mut diffuse = Colour::black();
        let mut specular = Colour::black();

        let light_positions = light.positions(rng);
        #[allow(clippy::cast_precision_loss)]
        let samples = light_positions.len() as f64;

        for light_position in light_positions {
            let light_vector = (light_position - *point).normalise();
            let light_dot_normal = light_vector.dot(normal);

            if light_dot_normal >= 0.0 {
                diffuse += colour * self.diffuse * light_dot_normal;

                let reflect_vector = -light_vector.reflect(normal);
                let reflect_dot_eye = reflect_vector.dot(eye);

                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(self.shininess);

                    specular += light.intensity() * self.specular * factor;
                };
            };
        }

        ambient + (diffuse + specular) / samples * intensity
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl_approx_eq!(&Material {
    ref pattern,
    ambient,
    diffuse,
    specular,
    shininess,
    reflective,
    transparency,
    refractive_index
});

impl<'de> Deserialize<'de> for Material {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Material {
            pattern: Option<Pattern>,
            #[serde(rename = "color")]
            colour: Option<Colour>,
            ambient: Option<f64>,
            diffuse: Option<f64>,
            specular: Option<f64>,
            shininess: Option<f64>,
            reflective: Option<f64>,
            transparency: Option<f64>,
            refractive_index: Option<f64>,
        }

        let material = Material::deserialize(deserializer)?;

        if material.pattern.is_some() && material.colour.is_some() {
            return Err(Error::custom(
                "Only one of pattern or colour can be set on a material",
            ));
        };

        let default = Self::default();

        let pattern = if let Some(pattern) = material.pattern {
            pattern
        } else if let Some(colour) = material.colour {
            colour.into()
        } else {
            default.pattern
        };

        Ok(Self::builder()
            .pattern(pattern)
            .ambient(material.ambient.unwrap_or(default.ambient))
            .diffuse(material.diffuse.unwrap_or(default.diffuse))
            .specular(material.specular.unwrap_or(default.specular))
            .shininess(material.shininess.unwrap_or(default.shininess))
            .reflective(material.reflective.unwrap_or(default.reflective))
            .transparency(material.transparency.unwrap_or(default.transparency))
            .refractive_index(
                material.refractive_index.unwrap_or(default.refractive_index),
            )
            .build())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use rand_xoshiro::Xoshiro256PlusPlus;
    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::float::*, object::Updatable, pattern::Pattern, world::test_world,
        Object,
    };

    fn rng() -> impl Rng {
        Xoshiro256PlusPlus::seed_from_u64(0)
    }

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

        let l = Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 0.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 10.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 10.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 0.0, 10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(&o, &l, &p, &e, &n, 1.0, &mut rng()),
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

        let l = Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let o = Object::test_builder().build();

        assert_approx_eq!(
            m.lighting(
                &o,
                &l,
                &Point::new(0.9, 0.0, 0.0),
                &e,
                &n,
                1.0,
                &mut rng()
            ),
            Colour::white()
        );

        assert_approx_eq!(
            m.lighting(
                &o,
                &l,
                &Point::new(1.1, 0.0, 0.0),
                &e,
                &n,
                1.0,
                &mut rng()
            ),
            Colour::black()
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn lighting_uses_light_intensity_to_attenuate_colour() {
        let mut w = test_world();

        w.lights[0] =
            Light::new_point(Point::new(0.0, 0.0, -10.0), Colour::white());
        let l = &w.lights[0];

        w.objects[0].replace_material(
            &Material::builder()
                .ambient(0.1)
                .diffuse(0.9)
                .specular(0.0)
                .build(),
        );
        let o = &w.objects[0];
        let m = &o.material();

        let p = Point::new(0.0, 0.0, -1.0);
        let e = -Vector::z_axis();
        let n = -Vector::z_axis();

        assert_approx_eq!(
            m.lighting(o, l, &p, &e, &n, 1.0, &mut rng()),
            Colour::white()
        );
        assert_approx_eq!(
            m.lighting(o, l, &p, &e, &n, 0.5, &mut rng()),
            Colour::new(0.55, 0.55, 0.55)
        );
        assert_approx_eq!(
            m.lighting(o, l, &p, &e, &n, 0.0, &mut rng()),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_samples_the_area_light() {
        let l = Light::new_area(
            Point::new(-0.5, -0.5, -5.0),
            Vector::x_axis(),
            2,
            Vector::y_axis(),
            2,
            Colour::white(),
        );

        let o = Object::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::white().into())
                    .ambient(0.1)
                    .diffuse(0.9)
                    .specular(0.0)
                    .build(),
            )
            .build();

        let e = Point::new(0.0, 0.0, -5.0);

        let test = |p: Point| {
            let e = (e - p).normalise();
            let n = Vector::new(p.x, p.y, p.z);

            o.material().lighting(&o, &l, &p, &e, &n, 1.0, &mut rng())
        };

        assert_approx_eq!(
            test(Point::new(0.0, 0.0, -1.0)),
            Colour::new(0.993_67, 0.993_67, 0.993_67),
            epsilon = 0.000_01
        );
        assert_approx_eq!(
            test(Point::new(0.0, SQRT_2, -SQRT_2)),
            Colour::new(0.780_05, 0.780_05, 0.780_05),
            epsilon = 0.000_01
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

    #[test]
    fn deserialize_material() {
        let m: Material = from_str(
            "\
color: [1, 0, 0]
ambient: 0.6
reflective: 0.5",
        )
        .unwrap();

        assert_approx_eq!(
            m,
            &Material::builder()
                .pattern(Colour::red().into())
                .ambient(0.6)
                .reflective(0.5)
                .build()
        );

        let m: Material = from_str(
            "\
pattern:
    kind: checker
    a: [1, 1, 1]
    b: [0, 0, 0]
diffuse: 0.3
specular: 0.5",
        )
        .unwrap();

        assert_approx_eq!(
            m,
            &Material::builder()
                .pattern(
                    Pattern::checker_builder(
                        Colour::white().into(),
                        Colour::black().into()
                    )
                    .build()
                )
                .diffuse(0.3)
                .specular(0.5)
                .build()
        );

        let m: Material = from_str(
            "\
shininess: 125.0
transparency: 0.4
refractive_index: 1.2",
        )
        .unwrap();

        assert_approx_eq!(
            m,
            &Material::builder()
                .shininess(125.0)
                .transparency(0.4)
                .refractive_index(1.2)
                .build()
        );
    }

    #[test]
    fn deserialize_invalid_material() {
        assert_eq!(
            from_str::<Material>(
                "
color: [1, 0, 0]
pattern:
    kind: checker
    a: [0, 1, 0]
    b: [0, 0, 1]"
            )
            .unwrap_err()
            .to_string(),
            "Only one of pattern or colour can be set on a material"
        );
    }
}
