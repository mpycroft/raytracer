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

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
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

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Colour::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_relative_eq!(c.pixels[32], red);
    }

    #[test]
    fn to_ppm() {
        let mut c = Canvas::new(5, 3);

        c.write_pixel(0, 0, Colour::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Colour::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Colour::new(-0.5, 0.0, 1.0));

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
0 127 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 0
0 0 255
"
        )
    }
}
