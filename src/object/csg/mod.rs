mod operation;

use float_cmp::{ApproxEq, F64Margin};

pub use self::operation::Operation;
use super::{Bounded, BoundingBox, Includes, Updatable};
use crate::{intersection::List, math::Transformation, Material, Object};

/// A `Csg` is a constructive solid geometry object which performs `Operations`
/// on its two operands allowing the combining of objects in different patterns.
#[derive(Clone, Debug)]
pub struct Csg {
    operation: Operation,
    left: Box<Object>,
    right: Box<Object>,
}

impl Csg {
    #[must_use]
    pub fn new(operation: Operation, left: Object, right: Object) -> Self {
        Self { operation, left: Box::new(left), right: Box::new(right) }
    }

    #[must_use]
    const fn intersection_allowed(
        &self,
        left_hit: bool,
        in_left: bool,
        in_right: bool,
    ) -> bool {
        match self.operation {
            Operation::Difference => {
                (left_hit && !in_right) || (!left_hit && in_left)
            }
            Operation::Intersection => {
                (left_hit && in_right) || (!left_hit && in_left)
            }

            Operation::Union => {
                (left_hit && !in_right) || (!left_hit && !in_left)
            }
        }
    }

    #[must_use]
    fn filter_intersections<'a>(&self, intersections: List<'a>) -> List<'a> {
        let mut in_left = false;
        let mut in_right = false;

        let mut list = List::new();

        for intersection in intersections.into_iter() {
            let left_hit = self.left.includes(intersection.object);

            if self.intersection_allowed(left_hit, in_left, in_right) {
                list.push(intersection);
            }

            if left_hit {
                in_left = !in_left;
            } else {
                in_right = !in_right;
            }
        }

        list
    }
}

impl Updatable for Csg {
    fn update_transformation(&mut self, transformation: &Transformation) {
        self.left.update_transformation(transformation);
        self.right.update_transformation(transformation);
    }

    fn replace_material(&mut self, material: &Material) {
        self.left.replace_material(material);
        self.right.replace_material(material);
    }
}

impl Bounded for Csg {
    #[must_use]
    fn bounding_box(&self) -> BoundingBox {
        self.left.bounding_box() + self.right.bounding_box()
    }
}

impl Includes for Csg {
    #[must_use]
    fn includes(&self, object: &Object) -> bool {
        if self.left.includes(object) || self.right.includes(object) {
            return true;
        }

        false
    }
}

impl ApproxEq for &Csg {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.operation == other.operation
            && self.left.approx_eq(&other.left, margin)
            && self.right.approx_eq(&other.right, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        intersection::Intersection,
        math::{float::*, Point},
    };

    #[test]
    fn creating_a_csg() {
        let l = Object::sphere_builder().build();
        let r = Object::test_builder().build();

        let o = Object::new_csg(Operation::Union, l.clone(), r.clone());

        let Object::Csg(c) = o else { unreachable!() };

        assert_eq!(c.operation, Operation::Union);
        assert_approx_eq!(c.left, &l);
        assert_approx_eq!(c.right, &r);
    }

    #[test]
    fn evaluating_the_rules_for_a_csg_operation() {
        let u = Object::new_csg(
            Operation::Union,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );

        let test = |o: &Object, l_hit, in_l, in_r| {
            let Object::Csg(c) = o else { unreachable!() };

            c.intersection_allowed(l_hit, in_l, in_r)
        };

        assert!(!test(&u, true, true, true));
        assert!(test(&u, true, true, false));
        assert!(!test(&u, true, false, true));
        assert!(test(&u, true, false, false));
        assert!(!test(&u, false, true, true));
        assert!(!test(&u, false, true, false));
        assert!(test(&u, false, false, true));
        assert!(test(&u, false, false, false));

        let i = Object::new_csg(
            Operation::Intersection,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );

        assert!(test(&i, true, true, true));
        assert!(!test(&i, true, true, false));
        assert!(test(&i, true, false, true));
        assert!(!test(&i, true, false, false));
        assert!(test(&i, false, true, true));
        assert!(test(&i, false, true, false));
        assert!(!test(&i, false, false, true));
        assert!(!test(&i, false, false, false));

        let d = Object::new_csg(
            Operation::Difference,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );

        assert!(!test(&d, true, true, true));
        assert!(test(&d, true, true, false));
        assert!(!test(&d, true, false, true));
        assert!(test(&d, true, false, false));
        assert!(test(&d, false, true, true));
        assert!(test(&d, true, true, false));
        assert!(!test(&d, false, false, true));
        assert!(!test(&d, false, false, false));
    }

    #[test]
    fn filtering_a_list_of_intersections() {
        let o1 = Object::sphere_builder().build();
        let o2 = Object::cube_builder().build();

        let l = List::from(vec![
            Intersection::new(&o1, 1.0),
            Intersection::new(&o2, 2.0),
            Intersection::new(&o1, 3.0),
            Intersection::new(&o2, 4.0),
        ]);

        let test = |o, i1: usize, i2: usize| {
            let o = Object::new_csg(o, o1.clone(), o2.clone());

            let Object::Csg(c) = o else { unreachable!() };

            let f = c.filter_intersections(l.clone());

            assert_eq!(f.len(), 2);
            assert_approx_eq!(f[0], l[i1]);
            assert_approx_eq!(f[1], l[i2]);
        };

        test(Operation::Union, 0, 3);
        test(Operation::Intersection, 1, 2);
        test(Operation::Difference, 0, 1);
    }

    #[test]
    fn the_bounding_box_of_a_csg() {
        let o = Object::new_csg(
            Operation::Intersection,
            Object::sphere_builder()
                .transformation(Transformation::new().translate(0.5, 0.0, 0.0))
                .build(),
            Object::cube_builder().build(),
        );

        assert_approx_eq!(
            o.bounding_box(),
            BoundingBox::new(
                Point::new(-1.0, -1.0, -1.0),
                Point::new(1.5, 1.0, 1.0)
            )
        );
    }

    #[test]
    fn test_updating_a_csg() {
        let mut o = Object::new_csg(
            Operation::Difference,
            Object::sphere_builder().build(),
            Object::test_builder().build(),
        );

        let t = Transformation::new().scale(2.0, 2.0, 2.0);

        o.update_transformation(&t);

        let m = Material::builder()
            .ambient(0.0)
            .diffuse(0.0)
            .reflective(1.0)
            .build();

        o.replace_material(&m);

        let Object::Csg(c) = o else { unreachable!() };
        let Object::Shape(s1) = *c.left else { unreachable!() };
        let Object::Shape(s2) = *c.right else { unreachable!() };

        assert_approx_eq!(s1.transformation, t);
        assert_approx_eq!(s2.transformation, t);

        assert_approx_eq!(s1.material, &m);
        assert_approx_eq!(s2.material, &m);
    }

    #[test]
    fn test_if_a_csg_includes_an_object() {
        let s = Object::sphere_builder().build();
        let cu = Object::cube_builder().build();
        let p = Object::plane_builder().build();

        let c = Object::new_csg(Operation::Difference, s.clone(), cu.clone());

        assert!(c.includes(&s));
        assert!(c.includes(&cu));
        assert!(!c.includes(&p));
    }

    #[test]
    fn comparing_csgs() {
        let c1 = Object::new_csg(
            Operation::Intersection,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );
        let c2 = Object::new_csg(
            Operation::Intersection,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );
        let c3 = Object::new_csg(
            Operation::Difference,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );

        assert_approx_eq!(c1, &c2);

        assert_approx_ne!(c1, &c3);
    }
}
