use derive_more::Constructor;
use float_cmp::{ApproxEq, F64Margin};

use crate::{math::Transformation, Material, Shape};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Copy, Debug, Constructor)]
pub struct Object {
    pub transformation: Transformation,
    pub material: Material,
    pub shape: Shape,
}

impl Object {
    #[must_use]
    pub fn new_test() -> Self {
        Self::new(Transformation::new(), Material::default(), Shape::new_test())
    }
}

impl ApproxEq for Object {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        self.shape == other.shape
            && self.transformation.approx_eq(other.transformation, margin)
            && self.material.approx_eq(other.material, margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, Colour};

    #[test]
    fn creating_an_object() {
        let t = Transformation::new().translate(2.0, 3.0, 0.0);
        let m = Material { colour: Colour::red(), ..Default::default() };
        let s = Shape::new_test();
        let o = Object::new(t, m, s);

        assert_approx_eq!(o.transformation, t);
        assert_approx_eq!(o.material, m);
        assert_eq!(o.shape, s);

        let o = Object::new_test();
        assert_approx_eq!(o.transformation, Transformation::new());
        assert_approx_eq!(o.material, Material::default());
        assert_eq!(o.shape, s);
    }

    #[test]
    fn comparing_objects() {
        let o1 = Object::new_test();
        let o2 = Object::new_test();
        let o3 = Object::new(
            Transformation::new().scale(1.0, 2.0, 1.0),
            Material::default(),
            Shape::new_test(),
        );

        assert_approx_eq!(o1, o2);

        assert_approx_ne!(o1, o3);
    }
}
