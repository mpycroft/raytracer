use super::Colour;

/// The Canvas represents the area we are going to be drawing images onto. This
/// will be a basic implementation and will probably need to be refactored later
/// on if we want to use parallel rendering or different image formats.
#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![Colour::black(); width * height] }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        self.pixels[y * self.width + x] = colour;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::assert_approx_eq;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for pixel in c.pixels {
            assert_approx_eq!(pixel, Colour::black());
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(2, 3, Colour::red());

        assert_approx_eq!(c.pixels[32], Colour::red());
    }
}
