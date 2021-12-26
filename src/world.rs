use crate::{
    intersect::{Computations, IntersectionList},
    math::{Point, Ray, Transform},
    Colour, Intersectable, Material, PointLight, Sphere,
};

/// World represents all the objects and light sources in a given scene that we
/// are rendering.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        World { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn push_object(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn push_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn shade_hit(&self, computations: &Computations) -> Colour {
        let mut colour = Colour::black();

        for light in &self.lights {
            colour += computations.object.material.lighting(
                light,
                &computations.over_point,
                &computations.eye,
                &computations.normal,
                self.is_shadowed(light, &computations.over_point),
            );
        }

        colour
    }

    pub fn colour_at(&self, ray: &Ray) -> Colour {
        if let Some(intersections) = self.intersect(ray) {
            if let Some(hit) = intersections.hit() {
                let computations = hit.prepare_computations(ray);

                return self.shade_hit(&computations);
            }
        }

        Colour::black()
    }

    pub fn is_shadowed(&self, light: &PointLight, point: &Point) -> bool {
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
}

impl Intersectable for World {
    fn intersect(&self, ray: &Ray) -> Option<IntersectionList> {
        let mut list = IntersectionList::new();

        for obj in &self.objects {
            if let Some(mut new_list) = obj.intersect(ray) {
                list.append(&mut *new_list);
            }
        }

        if list.is_empty() {
            None
        } else {
            list.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            Some(list)
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut world = World::new();

        world.push_object(Sphere::new(
            Transform::new(),
            Material::new(Colour::new(0.8, 1.0, 0.6), 0.1, 0.7, 0.2, 200.0),
        ));
        world.push_object(Sphere::new(
            Transform::from_scale(0.5, 0.5, 0.5),
            Material::default(),
        ));

        world.push_light(PointLight::new(
            Colour::white(),
            Point::new(-10.0, 10.0, -10.0),
        ));

        world
    }
}

#[cfg(test)]
mod tests {
    use approx::*;

    use super::*;
    use crate::{intersect::Intersection, math::Vector};

    #[test]
    fn new() {
        let w = World::new();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn push_object() {
        let mut w = World::new();

        let s = Sphere::default();
        w.push_object(s);

        assert_eq!(w.objects.len(), 1);
        assert_relative_eq!(w.objects[0], s);

        let s = Sphere::new(
            Transform::from_translate(-1.0, 2.3, 4.0),
            Material::default(),
        );
        w.push_object(s);

        assert_eq!(w.objects.len(), 2);
        assert_relative_eq!(w.objects[1], s);
    }

    #[test]
    fn push_light() {
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
    fn shade_hit() {
        let mut w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());
        let i = Intersection::new(&w.objects[0], 4.0);
        let c = i.prepare_computations(&r);

        assert_relative_eq!(
            w.shade_hit(&c),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );

        w.lights.clear();
        w.push_light(PointLight::new(
            Colour::white(),
            Point::new(0.0, 0.25, 0.0),
        ));

        let r = Ray::new(Point::origin(), Vector::z_axis());
        let i = Intersection::new(&w.objects[1], 0.5);
        let c = i.prepare_computations(&r);

        assert_relative_eq!(
            w.shade_hit(&c),
            Colour::new(0.904_984, 0.904_984, 0.904_984)
        );

        let mut w = World::new();

        w.push_light(PointLight::new(
            Colour::white(),
            Point::new(0.0, 0.0, -10.0),
        ));

        w.push_object(Sphere::default());

        w.push_object(Sphere::new(
            Transform::from_translate(0.0, 0.0, 10.0),
            Material::default(),
        ));

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::z_axis());
        let i = Intersection::new(&w.objects[1], 4.0);
        let c = i.prepare_computations(&r);

        assert_relative_eq!(w.shade_hit(&c), Colour::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn colour_at() {
        let mut w = World::default();
        let mut r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::y_axis());

        assert_relative_eq!(w.colour_at(&r), Colour::black());

        r.direction = Vector::z_axis();

        assert_relative_eq!(
            w.colour_at(&r),
            Colour::new(0.380_661, 0.475_826, 0.285_496)
        );

        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), -Vector::z_axis());

        assert_relative_eq!(w.colour_at(&r), w.objects[1].material.colour);
    }

    #[test]
    fn is_shadowed() {
        let w = World::default();

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(0.0, 10.0, 0.0)));

        assert!(w.is_shadowed(&w.lights[0], &Point::new(10.0, -10.0, 10.0)));

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-20.0, 20.0, -20.0)));

        assert!(!w.is_shadowed(&w.lights[0], &Point::new(-2.0, 2.0, -2.0)));
    }

    #[test]
    fn intersect() {
        let w = World::default();

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis());

        let list = w.intersect(&r);
        assert!(list.is_some());

        let list = list.unwrap();
        assert_float_relative_eq!(list[0].t, 4.0);
        assert_float_relative_eq!(list[1].t, 4.5);
        assert_float_relative_eq!(list[2].t, 5.5);
        assert_float_relative_eq!(list[3].t, 6.0);
    }

    #[test]
    fn default() {
        let w = World::default();

        assert_eq!(w.objects.len(), 2);
        assert_relative_eq!(
            w.objects[0],
            Sphere::new(
                Transform::new(),
                Material::new(Colour::new(0.8, 1.0, 0.6), 0.0, 0.7, 0.2, 0.0)
            )
        );
        assert_relative_eq!(
            w.objects[1],
            Sphere::new(
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
