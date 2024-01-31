/// `TValues` are the intersection values t and optionally u and v where an
/// intersection occurs.
#[derive(Clone, Copy, Debug)]
pub struct TValues {
    pub t: f64,
    pub u_v: Option<(f64, f64)>,
}

impl TValues {
    #[must_use]
    pub fn new(t: f64) -> Self {
        Self { t, u_v: None }
    }

    #[must_use]
    pub fn new_with_u_v(t: f64, u: f64, v: f64) -> Self {
        Self { t, u_v: Some((u, v)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_t_values() {
        let t = TValues::new(1.5);

        assert_approx_eq!(t.t, 1.5);
        assert!(t.u_v.is_none());

        let t = TValues::new_with_u_v(0.9, 2.1, 3.5);

        assert_approx_eq!(t.t, 0.9);

        let (u, v) = t.u_v.unwrap();
        assert_approx_eq!(u, 2.1);
        assert_approx_eq!(v, 3.5);
    }
}
