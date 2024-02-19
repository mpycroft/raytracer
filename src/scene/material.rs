use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::Deserialize;
use serde_yaml::{from_value, to_value, Value};

use super::Data;

/// A `Material` holds the deserialized material data.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Material {
    Name(String),
    Data(Value),
}

impl Material {
    pub fn parse(self, data: &Data) -> Result<crate::Material> {
        match self {
            Material::Name(name) => {
                if let Some(material) = data.materials.get(&name) {
                    material.clone().parse(data)
                } else {
                    bail!(
                        "Reference to material '{name}' that was not defined"
                    );
                }
            }
            Material::Data(data) => Ok(from_value(data)?),
        }
    }

    pub fn update(self, other: Value) -> Result<Self> {
        let mut material = match self {
            Material::Name(_) => unreachable!(),
            Material::Data(data) => from_value::<HashMap<String, Value>>(data)?,
        };

        let other = from_value::<HashMap<String, Value>>(other)?;

        material.extend(other);

        Ok(Material::Data(to_value(material)?))
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;
    use crate::{math::float::assert_approx_eq, Colour};

    #[test]
    fn parse_material() {
        let m: Material = from_str(
            "\
color: [1, 0, 0.5]
ambient: 0.4
specular: 0.7
shininess: 120",
        )
        .unwrap();

        let d = Data::new();

        let m = m.parse(&d).unwrap();

        assert_approx_eq!(
            m,
            &crate::Material::builder()
                .pattern(Colour::new(1.0, 0.0, 0.5).into())
                .ambient(0.4)
                .specular(0.7)
                .shininess(120.0)
                .build()
        );
    }

    #[test]
    fn parse_material_reference() {
        let m: Material = from_str("foo").unwrap();

        let mut d = Data::new();
        d.materials.insert(
            String::from("foo"),
            from_str(
                "\
color: [1, 1, 1]
diffuse: 0.0",
            )
            .unwrap(),
        );

        let m = m.parse(&d).unwrap();

        assert_approx_eq!(
            m,
            &crate::Material::builder()
                .pattern(Colour::white().into())
                .diffuse(0.0)
                .build()
        );

        let m: Material = from_str("bar").unwrap();

        assert_eq!(
            m.parse(&d).unwrap_err().to_string(),
            "Reference to material 'bar' that was not defined"
        );
    }

    #[test]
    fn update_material() {
        let m = Material::Data(
            from_str(
                "\
color: [1, 0, 1]
ambient: 0.4",
            )
            .unwrap(),
        );

        let m = m
            .update(
                from_str(
                    "\
ambient: 0.2
diffuse: 0.6",
                )
                .unwrap(),
            )
            .unwrap();

        let d = Data::new();

        let m = m.parse(&d).unwrap();
        assert_approx_eq!(
            m,
            &crate::Material::builder()
                .pattern(Colour::purple().into())
                .ambient(0.2)
                .diffuse(0.6)
                .build()
        );
    }
}
