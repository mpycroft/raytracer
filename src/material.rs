use derive_more::Constructor;

use crate::{math::float::impl_approx_eq, Colour};

/// A `Material` represents what a given object is made up of including what
/// colour it is and how it reacts to light.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Material {
    pub colour: Colour,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Colour::white(), 0.1, 0.9, 0.9, 200.0)
    }
}

impl_approx_eq!(Material { colour, ambient, diffuse, specular, shininess });

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn creating_a_material() {
        let m = Material::new(Colour::red(), 1.0, 1.0, 1.5, 25.6);

        assert_approx_eq!(m.colour, Colour::red());
        assert_approx_eq!(m.ambient, 1.0);
        assert_approx_eq!(m.diffuse, 1.0);
        assert_approx_eq!(m.specular, 1.5);
        assert_approx_eq!(m.shininess, 25.6);

        assert_approx_eq!(
            Material::default(),
            Material::new(Colour::white(), 0.1, 0.9, 0.9, 200.0)
        );
    }

    #[test]
    fn comparing_materials() {
        let m1 = Material::new(Colour::cyan(), 0.6, 0.3, 1.2, 142.7);
        let m2 = Material::new(Colour::cyan(), 0.6, 0.3, 1.2, 142.7);
        let m3 = Material::new(Colour::cyan(), 0.600_1, 0.3, 1.2, 142.7);

        assert_approx_eq!(m1, m2);

        assert_approx_ne!(m1, m3);
    }
}
