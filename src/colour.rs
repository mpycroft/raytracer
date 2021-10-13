/// A Colour represents an RGB colour in the image, values generally range from
/// 0.0..1.0 but can go outside this range before final processing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let c = Colour::new(-0.5, 0.4, 1.7);

        assert_float_relative_eq!(c.r, -0.5);
        assert_float_relative_eq!(c.g, 0.4);
        assert_float_relative_eq!(c.b, 1.7);
    }
}
