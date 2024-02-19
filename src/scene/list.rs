use anyhow::{bail, Result};
use serde::Deserialize;

use super::{Add, Data, Define};

/// An `Element` is either a deserialized definition or some object to add.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Element {
    Add(Add),
    Define(Define),
}

/// A `List` is the list of all elements that were deserialized.
#[derive(Clone, Debug, Deserialize)]
pub struct List(Vec<Element>);

impl List {
    pub fn parse(self, data: &mut Data) -> Result<()> {
        for element in self.0 {
            match element {
                Element::Add(add) => add.parse(data)?,
                Element::Define(define) => define.parse(data)?,
            }
        }

        if data.camera.is_none() {
            bail!("A camera must be defined")
        } else if data.lights.is_empty() {
            bail!("No lights were defined")
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn parse_list() {
        let l: List = from_str(
            "\
- define: foo
  value:
      add: cube
- add: light
  at: [-10, 10, -10]
  intensity: [1, 1, 1]
- add: camera
  width: 100
  height: 100
  field-of-view: 1.0
  from: [0, 0, 0]
  to: [0, 0, 5]
  up: [1, 0, 0]
- add: light
  corner: [10, -10, 10]
  uvec: [4, 0, 0]
  usteps: 4
  vvec: [0, 2, 0]
  vsteps: 2
  intensity: [0, 1, 0]",
        )
        .unwrap();

        let mut d = Data::new();

        l.parse(&mut d).unwrap();

        assert!(d.camera.is_some());
        assert_eq!(d.lights.len(), 2);
    }

    #[test]
    fn parse_no_camera() {
        let l: List = from_str(
            "\
- define: foo
  value:
      add: cube
- add: light
  at: [-10, 10, -10]
  intensity: [1, 1, 1]
- add: light
  corner: [10, -10, 10]
  uvec: [4, 0, 0]
  usteps: 4
  vvec: [0, 2, 0]
  vsteps: 2
  intensity: [0, 1, 0]",
        )
        .unwrap();

        let mut d = Data::new();

        assert_eq!(
            l.parse(&mut d).unwrap_err().to_string(),
            "A camera must be defined"
        );
    }

    #[test]
    fn parse_no_lights() {
        let l: List = from_str(
            "\
- define: foo
  value:
      add: cube
- add: camera
  width: 100
  height: 100
  field-of-view: 1.0
  from: [0, 0, 0]
  to: [0, 0, 5]
  up: [1, 0, 0]",
        )
        .unwrap();

        let mut d = Data::new();

        assert_eq!(
            l.parse(&mut d).unwrap_err().to_string(),
            "No lights were defined"
        );
    }
}
