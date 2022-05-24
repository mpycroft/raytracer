use super::Colour;
use crate::util::float::Float;

/// The `Canvas` represents the area we are going to be drawing images onto.
/// This will be a basic implementation and will probably need to be refactored
/// later on if we want to use parallel rendering or different image formats.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Canvas<T: Float> {
    width: usize,
    height: usize,
    pixels: Vec<Colour<T>>,
}

impl<T: Float> Canvas<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels: vec![Colour::default(); width * height] }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Colour<T> {
        self.pixels[y * self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour<T>) {
        self.pixels[y * self.width + x] = colour;
    }

    pub fn to_ppm(&self) -> String {
        let mut data = Vec::new();

        data.push(format!("P3\n{} {}\n255", self.width, self.height));

        for p in &self.pixels {
            let (r, g, b) = p.to_rgb();
            data.push(format!("{} {} {}", r, g, b));
        }

        // Make sure we have a final end line in the file
        data.push("".to_string());

        data.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::<f64>::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for p in c.pixels {
            assert_relative_eq!(p, Colour::black());
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::<f64>::new(10, 20);
        let red = Colour::red();

        c.write_pixel(2, 3, red);

        assert_relative_eq!(c.pixels[32], red);
    }

    #[test]
    fn reading_pixels_from_a_canvas() {
        let width = 10;
        let height = 10;
        let mut c = Canvas::new(width, height);

        for x in 0..width {
            for y in 0..height {
                c.write_pixel(
                    x,
                    y,
                    Colour::new(x as f64 * 0.1, y as f64 * 0.1, 0.0),
                );
            }
        }

        assert_relative_eq!(c.get_pixel(0, 0), Colour::black());
        assert_relative_eq!(c.get_pixel(3, 2), Colour::new(0.3, 0.2, 0.0));
        assert_relative_eq!(c.get_pixel(9, 9), Colour::new(0.9, 0.9, 0.0));
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::<f64>::new(5, 3);

        let ppm = c.to_ppm();
        let ppm = ppm.lines().take(3).collect::<Vec<_>>().join("\n");

        assert_eq!(
            ppm,
            "\
P3
5 3
255"
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        c.write_pixel(0, 0, Colour::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Colour::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Colour::new(-0.5, 0.0, 1.0));

        let ppm = c.to_ppm();
        let ppm = ppm.lines().skip(3).collect::<Vec<_>>().join("\n");

        assert_eq!(
            ppm,
            "\
255 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 127 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255"
        );
    }

    #[test]
    fn constructed_ppm_is_terminated_by_a_newline() {
        assert_eq!(
            Canvas::<f64>::new(5, 3).to_ppm().chars().last(),
            Some('\n')
        );
    }
}
