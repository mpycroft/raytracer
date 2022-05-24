use std::{
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

use num_traits::ToPrimitive;

/// Extend the `num_traits` version of `Float` with the various Assign operators
/// as they are not included by default. Also add Debug for convenience. While
/// this makes our types more restrictive than needed it doesn't matter for our
/// use case.
pub trait Float:
    num_traits::Float
    + AddAssign
    + DivAssign
    + MulAssign
    + SubAssign
    + Debug
    + Default
{
    fn two() -> Self {
        Self::one() + Self::one()
    }

    fn half() -> Self {
        Self::convert(0.5f64)
    }

    fn convert<U: ToPrimitive>(val: U) -> Self {
        Self::from(val).expect("Converting to Float failed")
    }
}

/// Blanket implementation of Float.
impl<T> Float for T where
    T: num_traits::Float
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign
        + Debug
        + Default
{
}
