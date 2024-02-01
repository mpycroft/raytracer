use enum_dispatch::enum_dispatch;

use crate::{math::Transformation, Material};

/// A trait that `Object` should implement to add another `Transformation` to
/// themselves or replace a `Material`. These should recursively be applied if
/// needed.
#[enum_dispatch(Object)]
pub trait Updatable {
    fn update_transformation(&mut self, transformation: &Transformation);

    fn replace_material(&mut self, material: &Material);
}
