use enum_dispatch::enum_dispatch;

use crate::Object;

/// A trait that determines if a given `Object` is part of another `Object`.
#[enum_dispatch(Object)]
pub trait Includes {
    fn includes(&self, object: &Object) -> bool;
}
