use serde::Deserialize;

/// `Operation` defines the various operations that can be performed between the
/// left and right children of a CSG.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Operation {
    Difference,
    Intersection,
    Union,
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;

    #[test]
    fn deserialize_operation() {
        let o: Operation = from_str("difference").unwrap();

        assert!(matches!(o, Operation::Difference));

        let o: Operation = from_str("intersection").unwrap();

        assert!(matches!(o, Operation::Intersection));

        let o: Operation = from_str("union").unwrap();

        assert!(matches!(o, Operation::Union));
    }
}
