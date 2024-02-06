use rand::prelude::*;

use crate::{
    intersection::{Computations, List},
    light::Lightable,
    math::{float::approx_eq, Point, Ray},
    Colour, Light, Object,
};

/// A `World` represents all the objects and light sources in a given scene that
/// we are rendering.
#[derive(Clone, Debug)]
pub struct World {
    pub(super) objects: Vec<Object>,
    pub(super) lights: Vec<Light>,
}

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    #[must_use]
    pub fn colour_at<R: Rng>(
        &self,
        ray: &Ray,
        depth: u32,
        rng: &mut R,
    ) -> Colour {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let computations =
                    hit.prepare_computations(ray, &intersections);

                return self.shade_hit(&computations, depth, rng);
            }
        }

        Colour::black()
    }

    #[must_use]
    pub fn shade_hit<R: Rng>(
        &self,
        computations: &Computations,
        depth: u32,
        rng: &mut R,
    ) -> Colour {
        let mut surface = Colour::black();

        for light in &self.lights {
            surface += computations.object.material().lighting(
                computations.object,
                light,
                &computations.over_point,
                &computations.eye,
                &computations.normal,
                light.intensity_at(&computations.over_point, self, rng),
                rng,
            );
        }

        let reflected = self.reflected_colour(computations, depth, rng);

        let refracted = self.refracted_colour(computations, depth, rng);

        if computations.object.material().reflective > 0.0
            && computations.object.material().transparency > 0.0
        {
            let reflectance = computations.schlick();

            return surface
                + reflected * reflectance
                + refracted * (1.0 - reflectance);
        }

        surface + reflected + refracted
    }

    #[must_use]
    fn intersect(&self, ray: &Ray) -> Option<List> {
        let mut list = List::new();

        for obj in &self.objects {
            if let Some(mut intersects) = obj.intersect(ray) {
                list.append(&mut *intersects);
            }
        }

        if list.is_empty() {
            return None;
        }

        list.sort();

        Some(list)
    }

    #[must_use]
    pub fn is_shadowed(&self, light_position: &Point, point: &Point) -> bool {
        let vector = *light_position - *point;

        let distance = vector.magnitude();
        let direction = vector.normalise();

        let ray = Ray::new(*point, direction);

        if let Some(intersections) = self.intersect(&ray) {
            if let Some(hit) = intersections.hit() {
                if hit.object.casts_shadow() && hit.t < distance {
                    return true;
                }
            }
        }

        false
    }

    #[must_use]
    pub fn reflected_colour<R: Rng>(
        &self,
        computations: &Computations,
        depth: u32,
        rng: &mut R,
    ) -> Colour {
        if depth == 0 || computations.object.material().reflective <= 0.0 {
            return Colour::black();
        }

        let reflect_ray =
            Ray::new(computations.over_point, computations.reflect);

        let colour = self.colour_at(&reflect_ray, depth - 1, rng);

        colour * computations.object.material().reflective
    }

    #[must_use]
    pub fn refracted_colour<R: Rng>(
        &self,
        computations: &Computations,
        depth: u32,
        rng: &mut R,
    ) -> Colour {
        if depth == 0
            || approx_eq!(computations.object.material().transparency, 0.0)
        {
            return Colour::black();
        }

        // Use Snell's Law to determine if we have total internal reflection.
        let n_ratio = computations.n1 / computations.n2;
        let cos_i = computations.eye.dot(&computations.normal);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return Colour::black();
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = computations.normal * (n_ratio * cos_i - cos_t)
            - computations.eye * n_ratio;

        let refracted_ray = Ray::new(computations.under_point, direction);

        self.colour_at(&refracted_ray, depth - 1, rng)
            * computations.object.material().transparency
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::module_name_repetitions)]
pub fn test_world() -> World {
    use crate::{math::Transformation, Material};

    let mut w = World::new();

    w.add_object(
        Object::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(0.8, 1.0, 0.6).into())
                    .diffuse(0.7)
                    .specular(0.2)
                    .build(),
            )
            .build(),
    );
    w.add_object(
        Object::sphere_builder()
            .transformation(Transformation::new().scale(0.5, 0.5, 0.5))
            .build(),
    );

    w.add_light(Light::new_point(
        Point::new(-10.0, 10.0, -10.0),
        Colour::white(),
    ));

    w
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, SQRT_2};

    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;
    use crate::{
        intersection::Intersection,
        math::{float::*, Angle, Transformation, Vector},
        object::Updatable,
        Camera, Material, Output, Pattern,
    };

    fn rng() -> impl Rng {
        Xoshiro256PlusPlus::seed_from_u64(0)
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

        let o1 = Object::test_builder().build();
        let o2 = Object::sphere_builder()
            .transformation(Transformation::new().translate(1.0, 2.0, 3.0))
            .build();

        w.add_object(o1.clone());
        w.add_object(o2.clone());

        assert_eq!(w.objects.len(), 2);
        assert_approx_eq!(w.objects[0], &o1);
        assert_approx_eq!(w.objects[1], &o2);

        let l1 = Light::new_point(Point::origin(), Colour::blue());
        let l2 = Light::new_point(Point::new(1.0, 2.0, 3.0), Colour::green());

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

        assert_approx_eq!(w.colour_at(&r, 5, &mut rng()), Colour::black());
    }

    #[test]
    fn the_colour_when_a_ray_hits() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        assert_approx_eq!(
            w.colour_at(&r, 2, &mut rng()),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_colour_with_an_intersection_behind_the_ray() {
        let mut w = test_world();

        w.objects[0].replace_material(
            &Material::builder()
                .pattern(Colour::new(0.8, 1.0, 0.6).into())
                .ambient(1.0)
                .diffuse(0.7)
                .specular(0.2)
                .build(),
        );
        w.objects[1]
            .replace_material(&Material::builder().ambient(1.0).build());

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), -Vector::z_axis());

        assert_approx_eq!(w.colour_at(&r, 1, &mut rng()), Colour::white());
    }

    #[test]
    fn colour_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();

        w.add_light(Light::new_point(Point::origin(), Colour::white()));

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
                .material(Material::builder().reflective(1.0).build())
                .build(),
        );
        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(Material::builder().reflective(1.0).build())
                .build(),
        );

        let r = Ray::new(Point::origin(), Vector::y_axis());

        let _ = w.colour_at(&r, 5, &mut rng());
    }

    #[test]
    fn shading_an_intersection() {
        let w = test_world();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let i = Intersection::new(&w.objects[0], 4.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5, &mut rng()),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = test_world();

        w.lights.clear();
        w.add_light(Light::new_point(
            Point::new(0.0, 0.25, 0.0),
            Colour::white(),
        ));

        let r = Ray::new(Point::origin(), Vector::z_axis());

        let i = Intersection::new(&w.objects[1], 0.5);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5, &mut rng()),
            Colour::new(0.904_98, 0.904_98, 0.904_98),
            epsilon = 0.000_01
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn colour_when_intersection_is_in_shadow() {
        let mut w = World::new();

        w.add_light(Light::new_point(
            Point::new(0.0, 0.0, -10.0),
            Colour::white(),
        ));

        w.add_object(Object::sphere_builder().build());

        let o = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, 10.0))
            .build();
        w.add_object(o.clone());

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());

        let i = Intersection::new(&o, 4.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 3, &mut rng()),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = test_world();

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(Material::builder().reflective(0.5).build())
                .build(),
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.shade_hit(&c, 5, &mut rng()),
            Colour::new(0.876_76, 0.924_34, 0.829_17),
            epsilon = 0.000_01
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn shade_hit_with_a_transparent_material() {
        let mut w = test_world();

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(
                    Material::builder()
                        .transparency(0.5)
                        .refractive_index(1.5)
                        .build(),
                )
                .build(),
        );
        w.add_object(
            Object::sphere_builder()
                .transformation(
                    Transformation::new().translate(0.0, -3.5, -0.5),
                )
                .material(
                    Material::builder()
                        .pattern(Colour::red().into())
                        .ambient(0.5)
                        .build(),
                )
                .build(),
        );

        let o = &w.objects[2];

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let l = List::from(Intersection::new(o, SQRT_2));

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(
            w.shade_hit(&c, 5, &mut rng()),
            Colour::new(0.936_43, 0.686_43, 0.686_43),
            epsilon = 0.000_01
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = test_world();

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(
                    Material::builder()
                        .reflective(0.5)
                        .transparency(0.5)
                        .refractive_index(1.5)
                        .build(),
                )
                .build(),
        );
        w.add_object(
            Object::sphere_builder()
                .transformation(
                    Transformation::new().translate(0.0, -3.5, -0.5),
                )
                .material(
                    Material::builder()
                        .pattern(Colour::red().into())
                        .ambient(0.5)
                        .build(),
                )
                .build(),
        );

        let o = &w.objects[2];

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let l = List::from(Intersection::new(o, SQRT_2));

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(
            w.shade_hit(&c, 5, &mut rng()),
            Colour::new(0.933_92, 0.696_43, 0.692_43),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = test_world();

        let i = w
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .unwrap();

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

        let mut o = Output::<Vec<_>>::new_sink();
        let i = c.render(&w, 5, true, &mut o, &mut rng()).unwrap();

        assert_approx_eq!(
            i.get_pixel(5, 5),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn rendering_a_world_multi_threaded() {
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

        let mut o = Output::<Vec<_>>::new_sink();
        let i = c.render(&w, 5, false, &mut o, &mut rng()).unwrap();

        assert_approx_eq!(
            i.get_pixel(5, 5),
            Colour::new(0.380_66, 0.475_83, 0.285_5),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn is_shadow_tests_for_occlusion_between_two_point() {
        let w = test_world();

        let l = Point::new(-10.0, -10.0, -10.0);

        assert!(!w.is_shadowed(&l, &Point::new(-10.0, -10.0, 10.0)));
        assert!(w.is_shadowed(&l, &Point::new(10.0, 10.0, 10.0)));
        assert!(!w.is_shadowed(&l, &Point::new(-20.0, -20.0, -20.0)));
        assert!(!w.is_shadowed(&l, &Point::new(-5.0, -5.0, 5.0)));
    }

    #[test]
    fn no_shadow_when_an_object_does_not_cast_shadow() {
        let mut w = test_world();

        w.objects[0] = Object::sphere_builder()
            .material(
                Material::builder()
                    .pattern(Colour::new(0.8, 1.0, 0.6).into())
                    .diffuse(0.7)
                    .specular(0.2)
                    .build(),
            )
            .casts_shadow(false)
            .build();

        assert!(!w.is_shadowed(
            &w.lights[0].positions(&mut rng())[0],
            &Point::new(10.0, -10.0, 10.0)
        ));
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_reflected_colour_for_a_non_reflective_material() {
        let mut w = test_world();

        w.objects[0].replace_material(
            &Material::builder()
                .pattern(Colour::new(0.8, 1.0, 0.6).into())
                .diffuse(0.7)
                .specular(0.2)
                .build(),
        );
        w.objects[1]
            .replace_material(&Material::builder().ambient(1.0).build());

        let r = Ray::new(Point::origin(), Vector::z_axis());

        let o = &w.objects[1];

        let i = Intersection::new(o, 1.0);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.reflected_colour(&c, 3, &mut rng()),
            Colour::black()
        );
    }

    #[test]
    fn the_reflected_colour_for_a_reflective_material() {
        let mut w = test_world();

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(Material::builder().reflective(0.5).build())
                .build(),
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.reflected_colour(&c, 4, &mut rng()),
            Colour::new(0.190_33, 0.237_91, 0.142_74),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_reflected_colour_at_the_maximum_recursion_depth() {
        let mut w = test_world();

        w.add_object(
            Object::plane_builder()
                .transformation(Transformation::new().translate(0.0, -1.0, 0.0))
                .material(Material::builder().reflective(0.5).build())
                .build(),
        );

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -sqrt_2_div_2, sqrt_2_div_2),
        );

        let i = Intersection::new(&w.objects[2], SQRT_2);

        let c = i.prepare_computations(&r, &List::from(i));

        assert_approx_eq!(
            w.reflected_colour(&c, 0, &mut rng()),
            Colour::black()
        );
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

        assert_approx_eq!(
            w.refracted_colour(&c, 5, &mut rng()),
            Colour::black()
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_refracted_colour_at_the_maximum_recursion_depth() {
        let mut w = test_world();

        w.objects[0].replace_material(
            &Material::builder()
                .pattern(Colour::new(0.8, 1.0, 0.6).into())
                .diffuse(0.7)
                .specular(0.2)
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        );

        let o = &w.objects[0];

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let l = List::from(vec![
            Intersection::new(o, 4.0),
            Intersection::new(o, 6.0),
        ]);

        let c = l[0].prepare_computations(&r, &l);

        assert_approx_eq!(
            w.refracted_colour(&c, 0, &mut rng()),
            Colour::black()
        );
    }

    #[test]
    #[allow(clippy::many_single_char_names)]
    fn the_refracted_colour_under_total_internal_reflection() {
        let mut w = test_world();

        w.objects[0].replace_material(
            &Material::builder()
                .pattern(Colour::new(0.8, 1.0, 0.6).into())
                .diffuse(0.7)
                .specular(0.2)
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        );

        let o = &w.objects[0];

        let sqrt_2_div_2 = SQRT_2 / 2.0;

        let r = Ray::new(Point::new(0.0, 0.0, sqrt_2_div_2), Vector::y_axis());

        let l = List::from(vec![
            Intersection::new(o, -sqrt_2_div_2),
            Intersection::new(o, sqrt_2_div_2),
        ]);

        let c = l[1].prepare_computations(&r, &l);

        assert_approx_eq!(
            w.refracted_colour(&c, 5, &mut rng()),
            Colour::black()
        );
    }

    #[test]
    fn the_refracted_colour_with_a_reflected_ray() {
        let mut w = test_world();

        w.objects[0].replace_material(
            &Material::builder()
                .pattern(Pattern::test_builder().build())
                .ambient(1.0)
                .build(),
        );
        w.objects[1].replace_material(
            &Material::builder()
                .transparency(1.0)
                .refractive_index(1.5)
                .build(),
        );

        let o1 = &w.objects[0];
        let o2 = &w.objects[1];

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::y_axis());

        let l = List::from(vec![
            Intersection::new(o1, -0.989_9),
            Intersection::new(o2, -0.489_9),
            Intersection::new(o2, 0.489_9),
            Intersection::new(o1, 0.989_9),
        ]);

        let c = l[2].prepare_computations(&r, &l);

        assert_approx_eq!(
            w.refracted_colour(&c, 5, &mut rng()),
            Colour::new(0.0, 0.998_88, 0.047_22),
            epsilon = 0.000_01
        );
    }
}
