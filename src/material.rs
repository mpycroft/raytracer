use crate::Colour;

/// Material represents what a given object is made up of including what colour
/// it is and how it reacts to light.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(
        colour: Colour,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
    ) -> Self {
        Self { colour, ambient, diffuse, specular, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Colour::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}

add_approx_traits!(Material { colour, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn new() {
        let c = Colour::new(0.5, 0.3, 0.0);
        let m = Material::new(c, 0.5, 1.0, 0.6, 100.0);

        assert_relative_eq!(m.colour, c);
        assert_float_relative_eq!(m.ambient, 0.5);
        assert_float_relative_eq!(m.diffuse, 1.0);
        assert_float_relative_eq!(m.specular, 0.6);
        assert_float_relative_eq!(m.shininess, 100.0);
    }

    #[test]
    fn default() {
        let m = Material::default();

        assert_relative_eq!(m.colour, Colour::new(1.0, 1.0, 1.0));
        assert_float_relative_eq!(m.ambient, 0.1);
        assert_float_relative_eq!(m.diffuse, 0.9);
        assert_float_relative_eq!(m.specular, 0.9);
        assert_float_relative_eq!(m.shininess, 200.0);
    }

    #[test]
    fn approx() {
        let m1 =
            Material::new(Colour::new(0.3, 0.4, 1.0), 0.2, 0.4, 0.3, 150.0);
        let m2 =
            Material::new(Colour::new(0.3, 0.4, 1.0), 0.2, 0.4, 0.3, 150.0);
        let m3 = Material::new(
            Colour::new(0.3, 0.4, 1.000_1),
            0.2,
            0.400_09,
            0.3,
            150.01,
        );

        assert_abs_diff_eq!(m1, m2);
        assert_abs_diff_ne!(m1, m3);

        assert_relative_eq!(m1, m2);
        assert_relative_ne!(m1, m3);

        assert_ulps_eq!(m1, m2);
        assert_ulps_ne!(m1, m3);
    }
}
