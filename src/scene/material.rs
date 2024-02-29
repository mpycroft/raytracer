use anyhow::{bail, Result};
use rand::prelude::*;
use serde::Deserialize;
use serde_yaml::{from_value, to_value, Value};

use super::{Data, HashValue, TransformationList};

/// A `Material` holds the deserialized material data.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Material {
    Name(String),
    Data(Value),
}

impl Material {
    pub fn parse<R: Rng>(
        self,
        data: &Data,
        rng: &mut R,
    ) -> Result<crate::Material> {
        let value = self.get_value(data)?;

        let mut hash_map: HashValue = from_value(value)?;

        if let Some(pattern) = hash_map.remove("pattern") {
            let mut pattern_hash_map: HashValue = from_value(pattern)?;

            pattern_hash_map
                .insert(String::from("seed"), to_value(rng.gen::<u64>())?);

            if let Some(transform) = pattern_hash_map.remove("transform") {
                let transformations: TransformationList =
                    from_value(transform)?;

                pattern_hash_map.insert(
                    String::from("transform"),
                    to_value(transformations.collect(data)?)?,
                );
            }

            hash_map
                .insert(String::from("pattern"), to_value(pattern_hash_map)?);
        }

        Ok(from_value(to_value(hash_map)?)?)
    }

    fn get_value(self, data: &Data) -> Result<Value> {
        match self {
            Self::Name(name) => {
                if let Some(material) = data.materials.get(&name) {
                    material.clone().get_value(data)
                } else {
                    bail!(
                        "Reference to material '{name}' that was not defined"
                    );
                }
            }
            Self::Data(data) => Ok(data),
        }
    }

    pub fn update(self, other: Value) -> Result<Self> {
        let mut material: HashValue = match self {
            Self::Name(_) => unreachable!(),
            Self::Data(data) => from_value(data)?,
        };

        let other: HashValue = from_value(other)?;

        material.extend(other);

        Ok(Self::Data(to_value(material)?))
    }
}

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoshiro256PlusPlus;
    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::{float::*, Angle, Transformation},
        Colour, Pattern,
    };

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

        let m = m.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();

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

        let mut r = Xoshiro256PlusPlus::seed_from_u64(0);
        let m = m.parse(&d, &mut r).unwrap();

        assert_approx_eq!(
            m,
            &crate::Material::builder()
                .pattern(Colour::white().into())
                .diffuse(0.0)
                .build()
        );

        let m: Material = from_str("bar").unwrap();

        assert_eq!(
            m.parse(&d, &mut r).unwrap_err().to_string(),
            "Reference to material 'bar' that was not defined"
        );
    }

    #[test]
    fn parse_material_with_transform_transformation() {
        let m: Material = from_str(
            "\
pattern:
    type: checker
    colors:
        - [1, 1, 1]
        - [0, 0, 0]
    transform:
         - [translate, 1, 2, 3]
         - foo
         - [rotate-x, 0.6]",
        )
        .unwrap();

        let mut d = Data::new();
        d.transformations.insert(
            String::from("foo"),
            from_str(
                "\
- [rotate-z, 1.0]
- [scale, 2, 2, 2]",
            )
            .unwrap(),
        );

        let m = m.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();

        assert_approx_eq!(
            m,
            &crate::Material::builder()
                .pattern(
                    Pattern::checker_builder(
                        Colour::white().into(),
                        Colour::black().into()
                    )
                    .transformation(
                        Transformation::new()
                            .translate(1.0, 2.0, 3.0)
                            .rotate_z(Angle(1.0))
                            .scale(2.0, 2.0, 2.0)
                            .rotate_x(Angle(0.6))
                    )
                    .build()
                )
                .build()
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

        let m = m.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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
