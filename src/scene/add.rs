use anyhow::{bail, Result};
use paste::paste;
use serde::Deserialize;
use serde_yaml::{from_value, Value};

use super::{
    shapes::{Cone, Cube, Cylinder, Plane, Sphere},
    Data,
};

/// The `Add` struct holds the deserialized data from an element in the Yaml
/// scene file.
#[derive(Clone, Debug, Deserialize)]
pub struct Add {
    add: String,
    #[serde(flatten)]
    value: Value,
}

impl Add {
    pub fn parse(self, data: &mut Data) -> Result<()> {
        macro_rules! map_to_object {
            ($name:literal) => {{
                paste! {
                    let object = from_value::<[<$name:camel>]>(
                        self.value
                    )?.parse(data)?;

                    data.objects.push(object);
                }
            }};
        }

        match &*self.add {
            "camera" => {
                if data.camera.is_some() {
                    bail!("Only one camera can be added")
                }

                data.camera = Some(from_value(self.value)?);
            }
            "light" => data.lights.push(from_value(self.value)?),
            "cone" => map_to_object!("cone"),
            "cube" => map_to_object!("cube"),
            "cylinder" => map_to_object!("cylinder"),
            "plane" => map_to_object!("plane"),
            "sphere" => map_to_object!("sphere"),
            _ => bail!("Unknown add '{self:?}'"),
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
            "
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
            "
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
            "
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
}
