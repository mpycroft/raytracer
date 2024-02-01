mod helper;

use float_cmp::{ApproxEq, F64Margin};

#[allow(clippy::module_name_repetitions)]
pub use self::helper::GroupBuilder;
use self::helper::Helper;
use super::{Bounded, BoundingBox, Object};
use crate::{
    intersection::List,
    math::{Ray, Transformation},
    Material,
};

/// A `Group` is a collection of `Object`s that can be treated as a single
/// entity.
#[derive(Clone, Debug)]
pub struct Group {
    pub(super) objects: Vec<Object>,
    bounding_box: BoundingBox,
}

impl Group {
    #[must_use]
    fn new(objects: Vec<Object>) -> Self {
        Self { objects, bounding_box: BoundingBox::default() }
    }

    pub fn builder() -> GroupBuilder {
        Helper::builder()
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        if !self.bounding_box.is_intersected_by(ray) {
            return None;
        }

        let mut list = List::new();

        for object in &self.objects {
            if let Some(object_list) = object.intersect(ray) {
                list.extend(object_list.iter());
            };
        }

        if list.is_empty() {
            return None;
        }

        Some(list)
    }

    pub(super) fn update_transformation(
        &mut self,
        transformation: &Transformation,
    ) {
        for object in &mut self.objects {
            object.update_transformation(transformation);
        }

        self.bounding_box = self.bounding_box();
    }

    pub(super) fn update_material(&mut self, material: &Material) {
        for object in &mut self.objects {
            object.update_material(material);
        }
    }
}

impl Bounded for Group {
    fn bounding_box(&self) -> BoundingBox {
        let mut bounding_box = BoundingBox::default();

        for object in &self.objects {
            bounding_box += object.bounding_box();
        }

        bounding_box
    }
}

impl ApproxEq for &Group {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        if self.objects.len() != other.objects.len() {
            return false;
        }

        let margin = margin.into();

        for (lhs, rhs) in self.objects.iter().zip(&other.objects) {
            if !lhs.approx_eq(rhs, margin) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use super::*;
    use crate::{
        intersection::Intersection,
        math::{float::*, Angle, Point, Transformation, Vector},
        Colour,
    };

    #[test]
    fn creating_a_group() {
        let o = Object::group_builder().build();

        let Object::Group(g) = o else { unreachable!() };

        assert_eq!(g.objects.len(), 0);

        let o = Object::group_builder()
            .set_objects(vec![
                Object::test_builder().build(),
                Object::test_builder().build(),
                Object::test_builder().build(),
            ])
            .build();

        let Object::Group(g) = o else { unreachable!() };

        assert_eq!(g.objects.len(), 3);

        let t1 = Object::test_builder().build();
        let t2 = Object::test_builder().build();

        let o = Object::group_builder().add_object(t1).add_object(t2).build();

        let Object::Group(g) = o else { unreachable!() };

        assert_eq!(g.objects.len(), 2);
    }

    #[test]
    fn intersecting_an_empty_group() {
        let o = Object::group_builder().build();

        assert!(o
            .intersect(&Ray::new(Point::origin(), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn intersecting_a_group_outside_its_bounding_box() {
        let o = Object::group_builder()
            .add_object(Object::sphere_builder().build())
            .build();

        assert!(o
            .intersect(&Ray::new(Point::new(2.0, 3.0, 5.0), Vector::z_axis()))
            .is_none());
    }

    #[test]
    fn intersecting_a_ray_with_a_non_empty_group() {
        let s1 = Object::sphere_builder().build();
        let s2 = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 0.0, -3.0))
            .build();
        let s3 = Object::sphere_builder()
            .transformation(Transformation::new().translate(5.0, 0.0, 0.0))
            .build();

        let o = Object::group_builder()
            .set_objects(vec![s1.clone(), s2.clone(), s3])
            .build();

        let l = o
            .intersect(&Ray::new(Point::new(0.0, 0.0, -5.0), Vector::z_axis()))
            .unwrap();
        assert_eq!(l.len(), 4);

        assert_approx_eq!(l[0].object, &s1);
        assert_approx_eq!(l[0].t, 4.0);
        assert_approx_eq!(l[1].object, &s1);
        assert_approx_eq!(l[1].t, 6.0);
        assert_approx_eq!(l[2].object, &s2);
        assert_approx_eq!(l[2].t, 1.0);
        assert_approx_eq!(l[3].object, &s2);
        assert_approx_eq!(l[3].t, 3.0);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let o = Object::group_builder()
            .add_object(
                Object::sphere_builder()
                    .transformation(
                        Transformation::new().translate(5.0, 0.0, 0.0),
                    )
                    .build(),
            )
            .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
            .build();

        let l = o
            .intersect(&Ray::new(
                Point::new(10.0, 0.0, -10.0),
                Vector::z_axis(),
            ))
            .unwrap();

        assert_eq!(l.len(), 2);

        assert_approx_eq!(l[0].t, 8.0);
        assert_approx_eq!(l[1].t, 12.0);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space_with_groups() {
        let o = Object::group_builder()
            .add_object(
                Object::group_builder()
                    .add_object(
                        Object::sphere_builder()
                            .transformation(
                                Transformation::new().translate(5.0, 0.0, 0.0),
                            )
                            .build(),
                    )
                    .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                    .build(),
            )
            .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
            .build();

        let Object::Group(g) = o else { unreachable!() };
        let Object::Group(g) = &g.objects[0] else { unreachable!() };
        let Object::Shape(s) = &g.objects[0] else { unreachable!() };

        assert_approx_eq!(
            s.to_object_space(&Point::new(-2.0, 0.0, -10.0)),
            Point::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space_with_groups() {
        let o = Object::group_builder()
            .add_object(
                Object::group_builder()
                    .add_object(
                        Object::sphere_builder()
                            .transformation(
                                Transformation::new().translate(5.0, 0.0, 0.0),
                            )
                            .build(),
                    )
                    .transformation(Transformation::new().scale(1.0, 2.0, 3.0))
                    .build(),
            )
            .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
            .build();

        let Object::Group(g) = o else { panic!() };
        let Object::Group(g) = &g.objects[0] else { panic!() };
        let Object::Shape(s) = &g.objects[0] else { panic!() };

        let sqrt_3_div_3 = f64::sqrt(3.0) / 3.0;
        assert_approx_eq!(
            s.to_world_space(&Vector::new(
                sqrt_3_div_3,
                sqrt_3_div_3,
                sqrt_3_div_3
            ))
            .normalise(),
            Vector::new(0.285_71, 0.428_57, -0.857_14),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let o = Object::group_builder()
            .add_object(
                Object::group_builder()
                    .add_object(
                        Object::sphere_builder()
                            .transformation(
                                Transformation::new().translate(5.0, 0.0, 0.0),
                            )
                            .build(),
                    )
                    .transformation(Transformation::new().scale(1.0, 2.0, 3.0))
                    .build(),
            )
            .transformation(Transformation::new().rotate_y(Angle(FRAC_PI_2)))
            .build();

        let Object::Group(g) = o else { unreachable!() };
        let Object::Group(g) = &g.objects[0] else { unreachable!() };
        let Object::Shape(s) = &g.objects[0] else { unreachable!() };

        let o = Object::test_builder().build();

        let i = Intersection::new(&o, 1.2);

        assert_approx_eq!(
            s.normal_at(&Point::new(1.732_1, 1.154_7, -5.577_4), &i),
            Vector::new(0.285_7, 0.428_54, -0.857_16),
            epsilon = 0.000_01
        );
    }

    #[test]
    fn the_bounding_box_of_a_group() {
        let o = Object::group_builder()
            .add_object(
                Object::cone_builder(1.0, 3.0, true)
                    .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                    .build(),
            )
            .transformation(Transformation::new().translate(1.0, 1.0, 0.0))
            .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(3.0, 3.0, 2.0),
                Point::new(7.0, 7.0, 6.0)
            )
        );
    }

    #[test]
    fn the_bounding_box_of_multiple_objects() {
        let o = Object::group_builder()
            .set_objects(vec![
                Object::sphere_builder()
                    .transformation(
                        Transformation::new().translate(3.0, 1.0, 3.0),
                    )
                    .build(),
                Object::cube_builder()
                    .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                    .build(),
            ])
            .transformation(Transformation::new().translate(-1.0, 0.0, 0.0))
            .build();

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(-3.0, -2.0, -2.0),
                Point::new(3.0, 2.0, 4.0)
            )
        );
    }

    #[test]
    fn a_groups_material_overwrites_objects() {
        let m = Material::builder()
            .pattern(Colour::new(0.4, 0.9, 1.0).into())
            .ambient(0.7)
            .diffuse(0.5)
            .transparency(0.0)
            .reflective(0.7)
            .refractive_index(0.0)
            .build();

        let g = Object::group_builder()
            .add_object(
                Object::group_builder()
                    .set_objects(vec![
                        Object::sphere_builder()
                            .material(Material::glass())
                            .build(),
                        Object::sphere_builder().build(),
                    ])
                    .build(),
            )
            .material(m.clone())
            .build();

        let Object::Group(g) = g else { unreachable!() };
        let Object::Group(g) = &g.objects[0] else { unreachable!() };
        let Object::Shape(s) = &g.objects[0] else { unreachable!() };

        assert_approx_eq!(s.material, &m);

        let Object::Shape(s) = &g.objects[1] else { unreachable!() };

        assert_approx_eq!(s.material, &m);
    }

    #[test]
    fn comparing_groups() {
        let g1 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g2 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g3 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g4 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder()
                .transformation(Transformation::new().translate(1.0, 2.0, 3.0))
                .build(),
        ]);

        assert_approx_eq!(g1, &g2);

        assert_approx_ne!(g1, &g3);
        assert_approx_ne!(g1, &g4);
    }
}
