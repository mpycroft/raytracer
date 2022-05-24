use crate::{
    intersect::{Computations, Intersectable, Intersection, IntersectionList},
    math::{Point, Ray, Transform},
    util::float::Float,
    Colour, Material, Object, Pattern, PointLight,
};

/// `World` represents all the objects and light sources in a given scene that
/// we are rendering.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct World<T: Float> {
    pub objects: Vec<Object<T>>,
    pub lights: Vec<PointLight<T>>,
}

impl<T: Float> World<T> {
    pub fn new() -> Self {
        Self { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn push_object(&mut self, object: Object<T>) {
        self.objects.push(object);
    }

    pub fn push_light(&mut self, light: PointLight<T>) {
        self.lights.push(light);
    }

    pub fn shade_hit(&self, computations: &Computations<T>) -> Colour<T> {
        let mut colour = Colour::black();

        for light in &self.lights {
            colour += computations.object.material.lighting(
                computations.object,
                light,
                &computations.over_point,
                &computations.eye,
                &computations.normal,
                self.is_shadowed(light, &computations.over_point),
            );
        }

        colour
    }

    pub fn colour_at(&self, ray: &Ray<T>) -> Colour<T> {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let computations = hit.prepare_computations(ray);

                return self.shade_hit(&computations);
            }
        }

        Colour::black()
    }

    pub fn is_shadowed(&self, light: &PointLight<T>, point: &Point<T>) -> bool {
        let v = light.position - *point;
        let distance = v.magnitude();
        let direction = v.normalise();

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

    fn intersect(&self, ray: &Ray<T>) -> Option<IntersectionList<T>> {
        let mut list = IntersectionList::new();

        for obj in &self.objects {
            if let Some(new_list) = obj.intersect(ray) {
                for t in new_list.iter() {
                    list.push(Intersection::new(obj, *t));
                }
            }
        }

        if list.is_empty() {
            None
        } else {
            list.sort_by(|a, b| {
                a.t.partial_cmp(&b.t).expect("Partial comparison failed")
            });
            Some(list)
        }
    }
}

impl<T: Float> Default for World<T> {
    fn default() -> Self {
        let mut world = Self::new();

        world.push_object(Object::new_sphere(
            Transform::new(),
            Material::new(
                Pattern::default_uniform(Colour::new(
                    T::convert(0.8f64),
                    T::one(),
                    T::convert(0.6f64),
                )),
                T::convert(0.1f64),
                T::convert(0.7f64),
                T::convert(0.2f64),
                T::convert(200.0f64),
            ),
        ));
        world.push_object(Object::new_sphere(
            Transform::from_scale(T::half(), T::half(), T::half()),
            Material::default(),
        ));

        world.push_light(PointLight::new(
            Colour::white(),
            Point::new(
                T::convert(-10.0f64),
                T::convert(10.0f64),
                T::convert(-10.0),
            ),
        ));

        world
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;
    use crate::math::Vector;

    #[test]
    fn creating_a_world() {
        let w = World::<f64>::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn adding_objects_to_a_world() {
        let mut w = World::new();

        let o = Object::default_sphere();
        w.push_object(o.clone());

        assert_eq!(w.objects.len(), 1);
        assert_relative_eq!(w.objects[0], o);

        let o = Object::new_sphere(
            Transform::from_translate(-1.0, 2.3, 4.0),
            Material::default(),
        );
        w.push_object(o.clone());

        assert_eq!(w.objects.len(), 2);
        assert_relative_eq!(w.objects[1], o);
    }

    #[test]
    fn adding_lights_to_a_world() {
        let mut w = World::new();

        let l = PointLight::new(Colour::red(), Point::origin());
        w.push_light(l);

        assert_eq!(w.lights.len(), 1);
        assert_relative_eq!(w.lights[0], l);

        let l = PointLight::new(Colour::green(), Point::new(1.0, 2.0, 3.0));
        w.push_light(l);

        assert_eq!(w.lights.len(), 2);
        assert_relative_eq!(w.lights[1], l);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();

        assert_relative_eq!(
            w.shade_hit(
                &Intersection::new(&w.objects[0], 4.0).prepare_computations(
                    &Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()),
                )
            ),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();

        w.lights.clear();
        w.push_light(PointLight::new(
            Colour::white(),
            Point::new(0.0, 0.25, 0.0),
        ));

        assert_relative_eq!(
            w.shade_hit(
                &Intersection::new(&w.objects[1], 0.5).prepare_computations(
                    &Ray::new(Point::origin(), Vector::z_axis())
                )
            ),
            Colour::new(0.904_984, 0.904_984, 0.904_984)
        );
    }

    #[test]
    fn shading_an_intersection_in_shadow() {
        let mut w = World::new();

        w.push_light(PointLight::new(
            Colour::white(),
            Point::new(0.0, 0.0, -10.0),
        ));

        w.push_object(Object::default_sphere());

        w.push_object(Object::new_sphere(
            Transform::from_translate(0.0, 0.0, 10.0),
            Material::default(),
        ));

        assert_relative_eq!(
            w.shade_hit(
                &Intersection::new(&w.objects[1], 4.0).prepare_computations(
                    &Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis()),
                )
            ),
            Colour::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn the_colour_when_a_ray_misses() {
        assert_relative_eq!(
            World::default().colour_at(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::y_axis()
            )),
            Colour::black()
        );
    }

    #[test]
    fn the_colour_when_a_ray_hits() {
        assert_relative_eq!(
            World::default().colour_at(&Ray::new(
                Point::new(0.0, 0.0, -5.0),
                Vector::z_axis()
            )),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );
    }

    #[test]
    fn the_colour_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        assert_relative_eq!(
            w.colour_at(&Ray::new(
                Point::new(0.0, 0.0, 0.75),
                -Vector::z_axis()
            )),
            Colour::white()
        );
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(0.0, 10.0, 0.0)));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();

        assert!(w.is_shadowed(&w.lights[0], &Point::new(10.0, -10.0, 10.0)));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-20.0, 20.0, -20.0)));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-2.0, 2.0, -2.0)));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let mut w = World::new();

        let o = Object::default_sphere();
        w.push_object(o.clone());

        let i = w
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .unwrap();

        assert_eq!(i.len(), 2);
        assert_relative_eq!(i[0].object, &o);
        assert_relative_eq!(i[1].object, &o);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();

        let list = w
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()));
        assert!(list.is_some());

        let list = list.unwrap();
        assert_float_relative_eq!(list[0].t, 4.0);
        assert_relative_eq!(*list[0].object, w.objects[0]);

        assert_float_relative_eq!(list[1].t, 4.5);
        assert_relative_eq!(*list[1].object, w.objects[1]);
        assert_float_relative_eq!(list[2].t, 5.5);
        assert_relative_eq!(*list[2].object, w.objects[1]);

        assert_float_relative_eq!(list[3].t, 6.0);
        assert_relative_eq!(*list[3].object, w.objects[0]);
    }

    #[test]
    fn the_default_world() {
        let w = World::default();

        assert_eq!(w.objects.len(), 2);
        assert_relative_eq!(
            w.objects[0],
            Object::new_sphere(
                Transform::new(),
                Material::new(
                    Pattern::default_uniform(Colour::new(0.8, 1.0, 0.6)),
                    0.1,
                    0.7,
                    0.2,
                    200.0
                )
            )
        );
        assert_relative_eq!(
            w.objects[1],
            Object::new_sphere(
                Transform::from_scale(0.5, 0.5, 0.5),
                Material::default()
            )
        );

        assert_eq!(w.lights.len(), 1);
        assert_relative_eq!(
            w.lights[0],
            PointLight::new(Colour::white(), Point::new(-10.0, 10.0, -10.0))
        );
    }
}
