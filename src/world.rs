use crate::{
    intersect::{Computations, Intersectable, IntersectionList},
    math::Ray,
    Colour, PointLight, Sphere,
};

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

    #[must_use]
    pub fn shade_hit(&self, computations: &Computations) -> Colour {
        let mut colour = Colour::black();

        for light in &self.lights {
            colour += computations.object.material.lighting(
                light,
                &computations.point,
                &computations.eye,
                &computations.normal,
            );
        }

        colour
    }

    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let mut list = IntersectionList::new();

        for obj in &self.objects {
            if let Some(mut intersects) = obj.intersect(ray) {
                list.append(&mut *intersects);
            }
        }

        if list.is_empty() {
            return None;
        }

        list.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Some(list)
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
        intersect::Intersection,
        math::{float::assert_approx_eq, Point, Transformation, Vector},
        Colour, Material,
    };

    fn test_world() -> World {
        let mut w = World::new();

        w.add_object(Sphere::new(
            Transformation::new(),
            Material {
                colour: Colour::new(0.8, 1.0, 0.6),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
        ));
        w.add_object(Sphere::new(
            Transformation::new().scale(0.5, 0.5, 0.5),
            Material::default(),
        ));

        w.add_light(PointLight::new(
            Point::new(-10.0, 10.0, -10.0),
            Colour::white(),
        ));

        w
    }

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

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = test_world();

        let i = w
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()));

        assert!(i.is_some());

        let i = i.unwrap();

        assert_eq!(i.len(), 4);
        assert_approx_eq!(i[0].t, 4.0);
        assert_approx_eq!(i[1].t, 4.5);
        assert_approx_eq!(i[2].t, 5.5);
        assert_approx_eq!(i[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let i = Intersection::new(&w.objects[0], 4.0);

        let c = i.prepare_computations(&r);

        assert_approx_eq!(
            w.shade_hit(&c),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = test_world();

        w.lights.clear();
        w.add_light(PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Colour::white(),
        ));

        let r = Ray::new(Point::origin(), Vector::z_axis());

        let i = Intersection::new(&w.objects[1], 0.5);

        let c = i.prepare_computations(&r);

        assert_approx_eq!(
            w.shade_hit(&c),
            Colour::new(0.904_98, 0.904_98, 0.904_98),
            epsilon = 0.000_01
        );
    }
}
