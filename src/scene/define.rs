use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::Deserialize;
use serde_yaml::{from_value, Value};

use super::{Data, TransformationList};

/// The `Define` struct holds the deserialized data of a definition that can be
/// referenced later on.
#[derive(Clone, Debug, Deserialize)]
pub struct Define {
    define: String,
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
        } else if let Ok(hash_map) =
            from_value::<HashMap<String, Value>>(self.value.clone())
        {
            if hash_map.contains_key("add") {
                if data.shapes.insert(self.define, self.value).is_some() {
                    bail!(err("Shape"));
                };
            } else if data.materials.insert(self.define, self.value).is_some() {
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
