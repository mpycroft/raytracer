use crate::{
    intersection::{Computations, Intersectable, List},
    math::{float::approx_eq, Point, Ray},
    Colour, Object, PointLight,
};

/// A `World` represents all the objects and light sources in a given scene that
/// we are rendering.
#[derive(Clone, Debug)]
pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    #[must_use]
    pub fn colour_at(&self, ray: &Ray, depth: u32) -> Colour {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let computations =
                    hit.prepare_computations(ray, &intersections);

                return self.shade_hit(&computations, depth);
            }
        }

        Colour::black()
    }

    #[must_use]
    pub fn shade_hit(&self, computations: &Computations, depth: u32) -> Colour {
        let mut surface = Colour::black();

        for light in &self.lights {
            surface += computations.object.material.lighting(
                computations.object,
                light,
                &computations.over_point,
                &computations.eye,
                &computations.normal,
                self.is_shadowed(light, &computations.over_point),
            );
        }

        let reflected = self.reflected_colour(computations, depth);

        surface + reflected
    }

    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<List> {
        let mut list = List::new();

        for obj in &self.objects {
            if let Some(intersects) = obj.intersect(ray) {
                list.append(&mut *intersects.build());
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

    #[must_use]
    pub fn reflected_colour(
        &self,
        computations: &Computations,
        depth: u32,
    ) -> Colour {
        if depth == 0 || computations.object.material.reflective <= 0.0 {
            return Colour::black();
        }

        let reflect_ray =
            Ray::new(computations.over_point, computations.reflect);

        let colour = self.colour_at(&reflect_ray, depth - 1);

        colour * computations.object.material.reflective
    }

    #[must_use]
    pub fn refracted_colour(
        &self,
        computations: &Computations,
        depth: u32,
    ) -> Colour {
        if depth == 0
            || approx_eq!(computations.object.material.transparency, 0.0)
        {
            return Colour::black();
        }

        // Use Snell's Law to determine if we have total internal reflection.
        let n_ratio = computations.n1 / computations.n2;
        let cos_i = computations.eye.dot(&computations.normal);
        let sin_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin_t > 1.0 {
            return Colour::black();
        }

        Colour::white()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, SQRT_2};

    use super::*;
    use crate::{
        math::{float::*, Angle, Transformation, Vector},
        Camera, Intersection, Material,
    };

    fn test_world() -> World {
        let mut w = World::new();

        w.add_object(Object::new_sphere(
            Transformation::new(),
            Material {
                pattern: Colour::new(0.8, 1.0, 0.6).into(),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
        ));
        w.add_object(Object::new_sphere(
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

        let o1 = Object::default_test();
        let o2 = Object::new_sphere(
            Transformation::new().translate(1.0, 2.0, 3.0),
            Material::default(),
        );

        w.add_object(o1.clone());
        w.add_object(o2.clone());

        assert_eq!(w.objects.len(), 2);
        assert_approx_eq!(w.objects[0], &o1);
        assert_approx_eq!(w.objects[1], &o2);

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

        assert_approx_eq!(w.colour_at(&r, 5), Colour::black());
    }

    #[test]
    fn the_colour_when_a_ray_hits() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        assert_approx_eq!(
            w.colour_at(&r, 2),
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

        assert_approx_eq!(w.colour_at(&r, 1), Colour::white());
    }

    #[test]
    fn colour_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();

        w.add_light(PointLight::new(Point::origin(), Colour::white()));

        w.add_object(Object::new_plane(
            Transformation::new().translate(0.0, 1.0, 0.0),
            Material { reflective: 1.0, ..Default::default() },
        ));
        w.add_object(Object::new_plane(
            Transformation::new().translate(0.0, -1.0, 0.0),
            Material { reflective: 1.0, ..Default::default() },
        ));

        let r = Ray::new(Point::origin(), Vector::y_axis());

        let _ = w.colour_at(&r, 5);
    }

    #[test]
    fn shading_an_intersection() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let i = Intersection::new(&w.objects[0], 4.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5),
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

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5),
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

        w.add_object(Object::default_sphere());

        let o = Object::new_sphere(
            Transformation::new().translate(0.0, 0.0, 10.0),
            Material::default(),
        );
        w.add_object(o.clone());

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let i = Intersection::new(&o, 4.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(w.shade_hit(&c, 3), Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = test_world();

        w.add_object(Object::new_plane(
            Transformation::new().translate(0.0, -1.0, 0.0),
            Material { reflective: 0.5, ..Default::default() },
        ));

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5),
            Colour::new(0.876_76, 0.924_34, 0.829_17),
            epsilon = 0.000_01
        );
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
            Angle(FRAC_PI_2),
            Transformation::view_transformation(
                &Point::new(0.0, 0.0, -5.0),
                &Point::origin(),
                &Vector::y_axis(),
            ),
        );

        let i = c.render(&w, 5, true);

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

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_reflected_colour_for_a_non_reflective_material() {
        let mut w = test_world();

        let r = Ray::new(Point::origin(), Vector::z_axis());

        w.objects[1].material.ambient = 1.0;
        let o = &w.objects[1];

        let i = Intersection::new(o, 1.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(w.reflected_colour(&c, 3), Colour::black());
    }

    #[test]
    fn the_reflected_colour_for_a_reflective_material() {
        let mut w = test_world();

        w.add_object(Object::new_plane(
            Transformation::new().translate(0.0, -1.0, 0.0),
            Material { reflective: 0.5, ..Default::default() },
        ));

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.reflected_colour(&c, 4),
            Colour::new(0.190_33, 0.237_91, 0.142_74),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_reflected_colour_at_the_maximum_recursion_depth() {
        let mut w = test_world();

        w.add_object(Object::new_plane(
            Transformation::new().translate(0.0, -1.0, 0.0),
            Material { reflective: 0.5, ..Default::default() },
        ));

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(w.reflected_colour(&c, 0), Colour::black());
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_refracted_colour_with_an_opaque_surface() {
        let w = test_world();

        let o = &w.objects[0];

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let l = List::from(vec![
            Intersection::new(o, 4.0),
            Intersection::new(o, 6.0),
        ]);

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(w.refracted_colour(&c, 5), Colour::black());
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_refracted_colour_at_the_maximum_recursion_depth() {
        let mut w = test_world();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;

        let o = &w.objects[0];

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let l = List::from(vec![
            Intersection::new(o, 4.0),
            Intersection::new(o, 6.0),
        ]);

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(w.refracted_colour(&c, 0), Colour::black());
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_refracted_colour_under_total_internal_reflection() {
        let mut w = test_world();
        w.objects[0].material.transparency = 1.0;
        w.objects[0].material.refractive_index = 1.5;

        let o = &w.objects[0];

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(Point::new(0.0, 0.0, sqrt_2_div_2), Vector::y_axis());

        let l = List::from(vec![
            Intersection::new(o, -sqrt_2_div_2),
            Intersection::new(o, sqrt_2_div_2),
        ]);

        let c = l[1].prepare_computations(&r, &l);

        assert_approx_eq!(w.refracted_colour(&c, 5), Colour::black());
    }
}
