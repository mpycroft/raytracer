use anyhow::{bail, Result};
use serde::Deserialize;
use serde_yaml::{from_value, Value};

use super::{shapes::parse_shape, Data};

/// The `Add` struct holds the deserialized data from an element in the Yaml
/// scene file.
#[derive(Clone, Debug, Deserialize)]
pub struct Add {
    pub add: String,
    #[serde(flatten)]
    pub value: Value,
}

impl Add {
    pub fn parse(self, data: &mut Data) -> Result<()> {
        match &*self.add {
            "camera" => {
                if data.camera.is_some() {
                    bail!("Only one camera can be added")
                }

                data.camera = Some(from_value(self.value)?);
            }
            "light" => data.lights.push(from_value(self.value)?),
            _ => data.objects.push(parse_shape(&self.add, self.value, data)?),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{consts::FRAC_PI_2, INFINITY, NEG_INFINITY};

    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::{float::assert_approx_eq, Angle, Point, Transformation, Vector},
        Camera, Colour, Light, Material, Object,
    };

    #[test]
    fn parse_camera() {
        let a: Add = from_str(
            "\
add: camera
width: 50
height: 50
field-of-view: \"PI / 2\"
from: [0, 2, -5]
to: [0, 0, 2]
up: [0, 1, 0]",
        )
        .unwrap();

        let mut d = Data::new();

        a.clone().parse(&mut d).unwrap();

        assert_approx_eq!(
            d.camera.unwrap(),
            Camera::new(
                50,
                50,
                Angle(FRAC_PI_2),
                Transformation::view_transformation(
                    Point::new(0.0, 2.0, -5.0),
                    Point::new(0.0, 0.0, 2.0),
                    Vector::y_axis()
                )
            )
        );

        assert_eq!(
            a.parse(&mut d).unwrap_err().to_string(),
            "Only one camera can be added"
        );
    }

    #[test]
    fn parse_light() {
        let a: Add = from_str(
            "\
add: light
at: [1, 1, 1]
intensity: [0, 0, 1]",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.lights.len(), 1);

        assert_approx_eq!(
            d.lights[0],
            Light::new_point(Point::new(1.0, 1.0, 1.0), Colour::blue())
        );
    }

    #[test]
    fn parse_cone() {
        let a: Add = from_str(
            "\
add: cone
max: 1.0
min: -1",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::cone_builder(-1.0, 1.0, false).build()
        );
    }

    #[test]
    fn parse_cube() {
        let a: Add = from_str(
            "\
add: cube
material:
    reflective: 0.8",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::cube_builder()
                .material(Material::builder().reflective(0.8).build())
                .build()
        );
    }

    #[test]
    fn parse_cylinder() {
        let a: Add = from_str("add: cylinder").unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::cylinder_builder(NEG_INFINITY, INFINITY, false).build()
        );
    }

    #[test]
    fn parse_group() {
        let a: Add = from_str(
            "\
add: group
children:
    - add: cube
      material:
          color: [0, 1, 0]
    - add: sphere
      transform:
          - [translate, 2, 2, 2]
material:
    color: [1, 0, 0]
transform:
    - [scale, 0.5, 0.5, 0.5]
shadow: false",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::group_builder()
                .set_objects(vec![
                    Object::cube_builder()
                        .material(
                            Material::builder()
                                .pattern(Colour::blue().into())
                                .build()
                        )
                        .build(),
                    Object::sphere_builder()
                        .transformation(
                            Transformation::new().translate(2.0, 2.0, 2.0)
                        )
                        .build()
                ])
                .material(
                    Material::builder().pattern(Colour::red().into()).build()
                )
                .transformation(Transformation::new().scale(0.5, 0.5, 0.5))
                .casts_shadow(false)
                .build()
        );
    }

    #[test]
    fn parse_plane() {
        let a: Add = from_str("add: plane").unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(d.objects[0], &Object::plane_builder().build());
    }

    #[test]
    fn parse_sphere() {
        let a: Add = from_str(
            "\
add: sphere
transform:
    - [scale, 2, 2, 2]",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::sphere_builder()
                .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                .build()
        );
    }

    #[test]
    fn parse_defined_shape() {
        let a: Add = from_str(
            "\
add: foo
transform:
    - [scale, 2, 2, 2]
material: bar",
        )
        .unwrap();

        let mut d = Data::new();
        d.shapes.insert(
            String::from("foo"),
            from_str(
                "\
add: sphere
transform:
    - [translate, 1, 1, 1]",
            )
            .unwrap(),
        );
        d.materials
            .insert(String::from("bar"), from_str("diffuse: 0.3").unwrap());

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::sphere_builder()
                .transformation(
                    Transformation::new()
                        .translate(1.0, 1.0, 1.0)
                        .scale(2.0, 2.0, 2.0)
                )
                .material(Material::builder().diffuse(0.3).build())
                .build()
        );

        let a: Add = from_str("add: bar").unwrap();

        assert_eq!(
            a.parse(&mut d).unwrap_err().to_string(),
            "Reference to shape 'bar' that was not defined"
        );
    }

    #[test]
    fn parse_csg() {
        let a: Add = from_str(
            "\
add: csg
operation: union
left:
    type: sphere
right:
    type: cube
    transform:
        - [scale, 1.5, 1.5, 1.5]",
        )
        .unwrap();

        let mut d = Data::new();

        a.parse(&mut d).unwrap();

        assert_eq!(d.objects.len(), 1);

        assert_approx_eq!(
            d.objects[0],
            &Object::new_csg(
                crate::Operation::Union,
                Object::sphere_builder().build(),
                Object::cube_builder()
                    .transformation(Transformation::new().scale(1.5, 1.5, 1.5))
                    .build()
            )
        );
    }
}
