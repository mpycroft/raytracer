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
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![Colour::black(); width * height] }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: &Colour) {
        self.pixels[y * self.width + x] = *colour;
    }

    #[must_use]
    pub fn to_ppm(&self) -> String {
        let mut data = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in &self.pixels {
            let (red, green, blue) = pixel.to_u8();

            data.push_str(&format!("{red} {green} {blue}\n"));
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

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
        c.write_pixel(2, 3, &Colour::red());

        assert_approx_eq!(c.pixels[32], Colour::red());
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 25 but the index is 53"
    )]
    fn writing_pixels_with_invalid_values() {
        let mut c = Canvas::new(5, 5);
        c.write_pixel(3, 10, &Colour::green());
    }

    #[test]
    fn generating_ppm_data_from_a_canvas() {
        let mut c = Canvas::new(5, 3);

        c.write_pixel(0, 0, &Colour::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, &Colour::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, &Colour::new(-0.5, 0.0, 1.0));

        assert_eq!(
            c.to_ppm(),
            "\
P3
5 3
255
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 128 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255\n"
        );

        let w = 10;
        let h = 2;
        let mut c = Canvas::new(w, h);

        for x in 0..w {
            for y in 0..h {
                c.write_pixel(x, y, &Colour::new(1.0, 0.8, 0.6));
            }
        }

        assert_eq!(
            c.to_ppm(),
            "\
P3
10 2
255
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153
255 204 153\n"
        );
    }
}
