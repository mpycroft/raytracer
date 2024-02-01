mod operation;

use float_cmp::{ApproxEq, F64Margin};
pub use operation::Operation;

use crate::Object;

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
        operation: Operation,
        left_hit: bool,
        in_left: bool,
        in_right: bool,
    ) -> bool {
        match operation {
            Operation::Difference => todo!(),
            Operation::Intersection => todo!(),
            Operation::Union => {
                (left_hit && !in_right) || (!left_hit && !in_left)
            }
        }
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
    use crate::math::float::*;

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
        let test_union = |l_hit, in_l, in_r| {
            Csg::intersection_allowed(Operation::Union, l_hit, in_l, in_r)
        };

        assert!(!test_union(true, true, true));
        assert!(test_union(true, true, false));
        assert!(!test_union(true, false, true));
        assert!(test_union(true, false, false));
        assert!(!test_union(false, true, true));
        assert!(!test_union(false, true, false));
        assert!(test_union(false, false, true));
        assert!(test_union(false, false, false));
    }
}
