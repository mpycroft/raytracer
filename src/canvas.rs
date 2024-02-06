use super::Colour;

/// The Canvas represents the area we are going to be drawing images onto. This
/// will be a basic implementation and will probably need to be refactored later
/// on if we want to use parallel rendering or different image formats.
#[derive(Clone, Debug)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Colour>,
}

impl Canvas {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Colour::black(); (width * height) as usize],
        }
    }

    /// Create a `Canvas` from an existing Vec<Colour>.
    ///
    /// # Panics
    ///
    /// Function will panic if passed a vector that contains less values than
    /// is required by the width and height.
    #[must_use]
    pub fn with_vec(width: u32, height: u32, pixels: Vec<Colour>) -> Self {
        assert_eq!(
            pixels.len(),
            (width * height) as usize,
            "Pixels must contain width * height values."
        );

        Self { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: &Colour) {
        self.pixels[y * self.width as usize + x] = *colour;
    }

    #[must_use]
    pub fn get_pixel(&self, x: usize, y: usize) -> Colour {
        self.pixels[y * self.width as usize + x]
    }

    #[must_use]
    pub fn to_ppm(&self) -> String {
        let mut data = format!("P3\n{} {}\n255\n", self.width, self.height);

        for pixel in &self.pixels {
            let [red, green, blue] = pixel.to_u8();

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

        let c = Canvas::with_vec(
            3,
            3,
            vec![
                Colour::red(),
                Colour::green(),
                Colour::blue(),
                Colour::red(),
                Colour::green(),
                Colour::blue(),
                Colour::red(),
                Colour::green(),
                Colour::blue(),
            ],
        );

        assert_eq!(c.width, 3);
        assert_eq!(c.height, 3);
        assert_approx_eq!(c.pixels[0], Colour::red());
        assert_approx_eq!(c.pixels[4], Colour::green());
        assert_approx_eq!(c.pixels[8], Colour::blue());
    }

    #[test]
    #[should_panic(expected = "\
assertion `left == right` failed: Pixels must contain width * height values.
  left: 1
 right: 100")]
    fn creating_a_canvas_with_invalid_vec() {
        let _ = Canvas::with_vec(10, 10, vec![Colour::black()]);
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
    fn getting_pixels_from_a_canvas() {
        let mut c = Canvas::new(10, 20);
        c.write_pixel(4, 4, &Colour::red());

        assert_approx_eq!(c.get_pixel(4, 4), Colour::red());
        assert_approx_eq!(c.get_pixel(3, 4), Colour::black());
        assert_approx_eq!(c.get_pixel(4, 3), Colour::black());
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 25 but the index is 35"
    )]
    fn getting_pixels_with_an_invalid_value() {
        let c = Canvas::new(5, 5);
        let _ = c.get_pixel(20, 3);
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
                c.write_pixel(
                    x as usize,
                    y as usize,
                    &Colour::new(1.0, 0.8, 0.6),
                );
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
