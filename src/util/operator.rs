/// Add left multiplication with a scaler for a given type where standard
/// multiplication is already implemented. We need to explicitly implement this
/// for f32 and f64 (rather than a generic T) because they are foreign types.
macro_rules! add_left_mul_scaler {
    (@impl $type:ty, $scaler:ty) => {
        impl<T: crate::util::float::Float> std::ops::Mul<$type> for $scaler {
            type Output = $type;

            fn mul(self, rhs: $type) -> Self::Output {
                rhs * T::from(self).unwrap()
            }
        }
    };
    ($type:ty) => {
        add_left_mul_scaler!(@impl $type, f32);
        add_left_mul_scaler!(@impl $type, f64);
    };
}
