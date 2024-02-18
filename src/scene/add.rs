use anyhow::{bail, Result};
use serde::Deserialize;
use serde_yaml::{from_value, Value};

use super::Data;

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
        match &*self.add {
            "camera" => data.camera = Some(from_value(self.value)?),
            "light" => data.lights.push(from_value(self.value)?),
            _ => bail!("Unknown add '{self:?}'"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::{float::assert_approx_eq, Angle, Point, Transformation, Vector},
        Camera, Colour, Light,
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

        a.parse(&mut d).unwrap();

        assert_approx_eq!(
            d.camera.unwrap(),
            Camera::new(
                50,
                50,
                Angle(FRAC_PI_2),
                Transformation::view_transformation(
                    &Point::new(0.0, 2.0, -5.0),
                    &Point::new(0.0, 0.0, 2.0),
                    &Vector::y_axis()
                )
            )
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

        assert_approx_eq!(
            d.lights[0],
            Light::new_point(Point::new(1.0, 1.0, 1.0), Colour::blue())
        );
    }
}
