use typed_builder::{Optional, TypedBuilder};

use super::{Group, Object};
use crate::{bounding_box::Bounded, math::Transformation};

#[derive(Clone, Debug, TypedBuilder)]
#[builder(build_method(vis = "", name = _build))]
pub struct Helper {
    #[builder(default = Transformation::new())]
    transformation: Transformation,
    #[builder(mutators(
        pub fn add_object(self, object: Object) {
            self.objects.push(object);
        }

        pub fn set_objects(self, objects: Vec<Object>) {
            self.objects = objects;
        }
    ))]
    #[builder(via_mutators)]
    objects: Vec<Object>,
}

impl<T: Optional<Transformation>> HelperBuilder<(T, (Vec<Object>,))> {
    #[must_use]
    pub fn build(self) -> Object {
        let mut group_helper = self._build();

        for object in &mut group_helper.objects {
            object.update_transformation(&group_helper.transformation);
        }

        let mut group = Group::new(group_helper.objects);
        group.bounding_box = group.bounding_box();

        group.into()
    }
}
