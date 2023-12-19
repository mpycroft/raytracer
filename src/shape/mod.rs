mod test;

pub use test::Test;

/// `Shape` is the list of the various geometries that can be rendered.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Test(Test),
}

impl Shape {
    #[must_use]
    pub fn new_test() -> Self {
        Self::Test(Test)
    }
}
