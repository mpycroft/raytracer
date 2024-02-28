use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_yaml::{from_value, to_value, Value};

use super::Data;
use crate::math::Transformation;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransformationList(Vec<Value>);

impl TransformationList {
    pub fn parse(self, data: &Data) -> Result<Transformation> {
        let transformations = self.collect(data)?;

        Ok(from_value(to_value(transformations)?)?)
    }

    pub fn collect(self, data: &Data) -> Result<Self> {
        let mut final_transformations = Vec::new();

        for transformation in self.0 {
            if let Some(define) = transformation.as_str() {
                if let Some(transformations) = data.transformations.get(define)
                {
                    final_transformations
                        .extend(transformations.clone().collect(data)?.0);
                } else {
                    bail!("Reference to transformations '{define}' that was not defined")
                }
            } else {
                final_transformations.push(transformation);
            }
        }

        Ok(Self(final_transformations))
    }

    pub fn combine(lhs: Value, rhs: Value) -> Result<Value> {
        let mut transformations: Self = from_value(lhs)?;

        let rhs: Self = from_value(rhs)?;

        transformations.0.extend(rhs.0);

        Ok(to_value(transformations)?)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_3;

    use serde_yaml::from_str;

    use super::*;
    use crate::math::{float::*, Angle};

    #[test]
    fn parse_simple_transformations() {
        let t: TransformationList = from_str(
            "\
- [scale, 2, 2, 0.5]
- [translate, 0, 1.3, 0]",
        )
        .unwrap();

        let d = Data::new();

        let t = t.parse(&d).unwrap();

        assert_approx_eq!(
            t,
            Transformation::new().scale(2.0, 2.0, 0.5).translate(0.0, 1.3, 0.0)
        );
    }

    #[test]
    fn parse_defined_transformations() {
        let t: TransformationList = from_str(
            "\
- foo
- [scale, 1, 0.5, 1.5]
- bar
- [translate, 1, 2, 3]",
        )
        .unwrap();

        let mut d = Data::new();
        d.transformations.insert(
            String::from("foo"),
            from_str(
                "\
- [rotate-x, 0.5]
- [translate, 3, 3, 3]",
            )
            .unwrap(),
        );
        d.transformations.insert(
            String::from("bar"),
            from_str(
                "\
- [rotate-y, \"PI / 3\"]
- baz",
            )
            .unwrap(),
        );
        d.transformations.insert(
            String::from("baz"),
            from_str("- [rotate-z, 1.0]").unwrap(),
        );

        let t = t.parse(&d).unwrap();

        assert_approx_eq!(
            t,
            Transformation::new()
                .rotate_x(Angle(0.5))
                .translate(3.0, 3.0, 3.0)
                .scale(1.0, 0.5, 1.5)
                .rotate_y(Angle(FRAC_PI_3))
                .rotate_z(Angle(1.0))
                .translate(1.0, 2.0, 3.0)
        );

        let t: TransformationList = from_str("- qux").unwrap();

        assert_eq!(
            t.parse(&d).unwrap_err().to_string(),
            "Reference to transformations 'qux' that was not defined"
        );
    }
}
