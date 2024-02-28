/// Macro to implement serde Deserialize for a type that can be represented as 3 f64's.
macro_rules! impl_deserialize_tuple {
    ($ty:ty) => {
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let [a, b, c] = <[f64; 3]>::deserialize(deserializer)?;

                Ok(Self::new(a, b, c))
            }
        }
    };
}
pub(crate) use impl_deserialize_tuple;
