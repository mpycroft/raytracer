use typed_builder::{Optional, TypedBuilder};

use super::Object;
use crate::math::Transformation;

#[derive(Clone, Debug, TypedBuilder)]
#[builder(build_method(vis = "", name = _build))]
pub struct GroupHelper {
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

impl<T: Optional<Transformation>> GroupHelperBuilder<(T, (Vec<Object>,))> {
    #[must_use]
    pub fn build(self) -> Object {
        let mut group = self._build();

        for object in group.objects {
                child_object.transformation =
                    child_object.transformation.extend(&object.transformation);
                child_object.inverse_transformation =
                    child_object.transformation.invert();
                child_object.bounding_box = child_object.bounding_box();
            }

            object.transformation = Transformation::new();
            object.inverse_transformation = Transformation::new();

            group.update_bounding_box();
        };

        object.bounding_box = object.bounding_box();

        object.into()
    }
}
