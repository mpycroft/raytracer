use super::Colour;

/// The Canvas represents the area we are going to be drawing images onto. This
/// will be a basic implementation and will probably need to be refactored later
/// on if we want to use parallel rendering or different image formats.
#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![Colour::default(); width * height] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for p in c.pixels {
            assert_relative_eq!(p, Colour::new(0.0, 0.0, 0.0));
        }
    }
}
