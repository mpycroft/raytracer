mod operation;

use float_cmp::{ApproxEq, F64Margin};
pub use operation::Operation;

use super::Includes;
use crate::{intersection::List, Object};

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
    fn intersection_allowed(
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

impl Includes for Csg {
    #[must_use]
    fn includes(&self, object: &Object) -> bool {
        if self.left.includes(object) || self.right.includes(object) {
            return true;
        }

        false
    }
}

impl ApproxEq for Csg {
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
    use crate::{intersection::Intersection, math::float::*};

    #[test]
    fn creating_a_csg() {
        let l = Object::sphere_builder().build();
        let r = Object::test_builder().build();

        let c = Csg::new(Operation::Union, l.clone(), r.clone());

        assert_eq!(c.operation, Operation::Union);
        assert_approx_eq!(c.left, &l);
        assert_approx_eq!(c.right, &r);
    }

    #[test]
    fn evaluating_the_rules_for_a_csg_operation() {
        let u = Csg::new(
            Operation::Union,
            Object::test_builder().build(),
            Object::test_builder().build(),
        );

        let test = |c: &Csg, l_hit, in_l, in_r| {
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

        let i = Csg::new(
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

        let d = Csg::new(
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
            let c = Csg::new(o, o1.clone(), o2.clone());

            let f = c.filter_intersections(l.clone());

            assert_eq!(f.len(), 2);
            assert_approx_eq!(f[0], l[i1]);
            assert_approx_eq!(f[1], l[i2]);
        };

        test(Operation::Union, 0, 3);
        test(Operation::Intersection, 1, 2);
        test(Operation::Difference, 0, 1);
    }
}
