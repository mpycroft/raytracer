use crate::{PointLight, Sphere};

/// A `World` represents all the objects and light sources in a given scene that
/// we are rendering.
#[derive(Clone, Debug)]
pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<PointLight>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn add_object(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{float::assert_approx_eq, Point, Transformation},
        Colour, Material,
    };

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);

        let w = World::default();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn adding_elements_to_a_world() {
        let mut w = World::new();

        let s1 = Sphere::default();
        let s2 = Sphere::new(
            Transformation::new().translate(1.0, 2.0, 3.0),
            Material::default(),
        );

        w.add_object(s1);
        w.add_object(s2);

        assert_eq!(w.objects.len(), 2);
        assert_approx_eq!(w.objects[0], s1);
        assert_approx_eq!(w.objects[1], s2);

        let l1 = PointLight::new(Point::origin(), Colour::blue());
        let l2 = PointLight::new(Point::new(1.0, 2.0, 3.0), Colour::green());

        w.add_light(l1);
        w.add_light(l2);

        assert_eq!(w.lights.len(), 2);
        assert_approx_eq!(w.lights[0], l1);
        assert_approx_eq!(w.lights[1], l2);
    }
}
