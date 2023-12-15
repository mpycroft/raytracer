use crate::{
    intersect::{Computations, Intersectable, IntersectionList},
    math::{Point, Ray},
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
    pub fn colour_at(&self, ray: &Ray) -> Colour {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let computations = hit.prepare_computations(ray);

                return self.shade_hit(&computations);
            }
        }

        Colour::black()
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
                self.is_shadowed(light, &computations.point),
            );
        }

        colour
    }

    #[must_use]
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

    #[must_use]
    pub fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
        let vector = light.position - *point;

        let distance = vector.magnitude();
        let direction = vector.normalise();

        let ray = Ray::new(*point, direction);

        if let Some(intersections) = self.intersect(&ray) {
            if let Some(hit) = intersections.hit() {
                if hit.t < distance {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::*;
    use crate::{
        intersect::Intersection,
        math::{float::assert_approx_eq, Transformation, Vector},
        Camera, Material,
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
    fn the_colour_when_a_ray_misses() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::y_axis());

        assert_approx_eq!(w.colour_at(&r), Colour::black());
    }

    #[test]
    fn the_colour_when_a_ray_hits() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        assert_approx_eq!(
            w.colour_at(&r),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_colour_with_an_intersection_behind_the_ray() {
        let mut w = test_world();

        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), -Vector::z_axis());

        assert_approx_eq!(w.colour_at(&r), w.objects[1].material.colour);
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

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn colour_when_intersection_is_in_shadow() {
        let mut w = World::new();

        w.add_light(PointLight::new(
            Point::new(0.0, 0.0, -10.0),
            Colour::white(),
        ));

        w.add_object(Sphere::default());

        let s = Sphere::new(
            Transformation::new().translate(0.0, 0.0, 10.0),
            Material::default(),
        );
        w.add_object(s);

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let i = Intersection::new(&s, 4.0);

        let c = i.prepare_computations(&r);

        assert_approx_eq!(w.shade_hit(&c), Colour::new(0.1, 0.1, 0.1));
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
    fn rendering_a_world_with_a_camera() {
        let w = test_world();
        let c = Camera::new(
            11,
            11,
            FRAC_PI_2,
            Transformation::view_transformation(
                &Point::new(0.0, 0.0, -5.0),
                &Point::origin(),
                &Vector::y_axis(),
            ),
        );

        let i = c.render(&w);

        assert_approx_eq!(
            i.get_pixel(5, 5),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = test_world();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(0.0, 10.0, 0.0)));
    }

    #[test]
    fn shadow_when_an_object_is_between_point_and_light() {
        let w = test_world();

        assert!(w.is_shadowed(&w.lights[0], &Point::new(10.0, -10.0, 10.0)));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_light() {
        let w = test_world();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-20.0, 20.0, -20.0)));
    }

    #[test]
    fn no_shadow_when_an_object_is_behind_the_point() {
        let w = test_world();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-2.0, 2.0, -2.0)));
    }
}
