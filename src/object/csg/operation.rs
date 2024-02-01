/// `Operation` defines the various operations that can be performed between the
/// left and right children of a CSG.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
    Difference,
    Intersection,
    Union,
}
