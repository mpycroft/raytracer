use anyhow::{bail, Result};
use serde::Deserialize;
use serde_yaml::{from_value, Value};

use super::{Add, Data, HashValue, Material, TransformationList};

/// The `Define` struct holds the deserialized data of a definition that can be
/// referenced later on.
#[derive(Clone, Debug, Deserialize)]
pub struct Define {
    #[allow(clippy::struct_field_names)]
    define: String,
    extend: Option<String>,
    value: Value,
}

impl Define {
    pub fn parse(self, data: &mut Data) -> Result<()> {
        let self_name = self.define.clone();

        let err = |name| format!("{name} '{self_name}' already defined");

        if let Ok(transformations) =
            from_value::<TransformationList>(self.value.clone())
        {
            if data
                .transformations
                .insert(self.define, transformations)
                .is_some()
            {
                bail!(err("Transformations"));
            }
        } else if let Ok(add) = from_value::<Add>(self.value.clone()) {
            if data.shapes.insert(self.define, add).is_some() {
                bail!(err("Shape"));
            };
        } else if from_value::<HashValue>(self.value.clone()).is_ok() {
            let material = if let Some(extend) = self.extend {
                if let Some(define) = data.materials.get(&extend) {
                    define.clone().update(self.value)?
                } else {
                    bail!(
                        "\
Attempt to extend material '{extend}' which was not defined"
                    )
                }
            } else {
                Material::Data(self.value)
            };

            if data.materials.insert(self.define, material).is_some() {
                bail!(err("Material"));
            }
        } else {
            bail!("Unable to parse define '{}'", self.define)
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;
    use crate::math::float::*;

    #[test]
    fn parse_define_shape() {
        let d: Define = from_str(
            "\
define: foo
value:
    add: cube",
        )
        .unwrap();

        let mut da = Data::new();

        d.clone().parse(&mut da).unwrap();

        assert_eq!(da.shapes.len(), 1);

        assert_eq!(
            d.parse(&mut da).unwrap_err().to_string(),
            "Shape 'foo' already defined"
        );
    }

    #[test]
    fn parse_define_material() {
        let d: Define = from_str(
            "\
define: foo
value:
    colour: [1, 1, 1]",
        )
        .unwrap();

        let mut da = Data::new();

        d.clone().parse(&mut da).unwrap();

        assert_eq!(da.materials.len(), 1);

        assert_eq!(
            d.parse(&mut da).unwrap_err().to_string(),
            "Material 'foo' already defined"
        );
    }

    #[test]
    fn define_extend_material() {
        let d: Define = from_str(
            "\
define: bar
extend: foo
value:
    color: [1, 1, 1]",
        )
        .unwrap();

        let mut da = Data::new();
        da.materials.insert(
            String::from("foo"),
            Material::Data(
                from_str::<Value>(
                    "\
color: [1, 0, 0]
ambient: 0.1",
                )
                .unwrap(),
            ),
        );

        d.parse(&mut da).unwrap();

        assert_eq!(da.materials.len(), 2);

        let Material::Data(d) = da.materials.get("bar").unwrap() else {
            unreachable!()
        };

        assert_approx_eq!(
            from_value::<crate::Material>(d.clone()).unwrap(),
            &from_value::<crate::Material>(
                from_str::<Value>(
                    "\
color: [1, 1, 1]
ambient: 0.1"
                )
                .unwrap()
            )
            .unwrap()
        );

        let d: Define = from_str(
            "\
define: baz
extend: qux
value:
    diffuse: 1.0",
        )
        .unwrap();

        assert_eq!(
            d.parse(&mut da).unwrap_err().to_string(),
            "Attempt to extend material 'qux' which was not defined"
        );
    }

    #[test]
    fn define_transformation() {
        let d: Define = from_str(
            "
define: foo
value:
    - [translate, 1, 1, 1]",
        )
        .unwrap();

        let mut da = Data::new();

        d.clone().parse(&mut da).unwrap();

        assert_eq!(da.transformations.len(), 1);

        assert_eq!(
            d.parse(&mut da).unwrap_err().to_string(),
            "Transformations 'foo' already defined"
        );
    }

    #[test]
    fn invalid_define() {
        let d: Define = from_str(
            "
define: foo
value: true",
        )
        .unwrap();

        let mut da = Data::new();

        assert_eq!(
            d.parse(&mut da).unwrap_err().to_string(),
            "Unable to parse define 'foo'"
        );
    }
}
