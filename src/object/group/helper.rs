use typed_builder::{Optional, TypedBuilder};

use super::{BoundingBox, Group, Object, Updatable};
use crate::{math::Transformation, Material};

pub type GroupBuilder = HelperBuilder<((), (), (Vec<Object>,))>;

/// This is a helper struct for constructing `Groups`, since we don't actually
/// store the transformation or material for a group but do use them to "push
/// down" the values to children we don't want them in the actual `Group` struct
/// itself.
#[derive(Clone, Debug, TypedBuilder)]
#[builder(builder_method(vis = "pub(super)"))]
#[builder(build_method(vis = "", name = _build))]
pub struct Helper {
    #[builder(default = Transformation::new())]
    transformation: Transformation,
    #[builder(default = None, setter(strip_option))]
    material: Option<Material>,
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

impl<T, M> HelperBuilder<(T, M, (Vec<Object>,))>
where
    T: Optional<Transformation>,
    M: Optional<Option<Material>>,
{
    #[must_use]
    pub fn build(self) -> Object {
        let group_helper = self._build();

        let transformation = group_helper.transformation;
        let material = group_helper.material;

        let mut group = Group {
            objects: group_helper.objects,
            bounding_box: BoundingBox::default(),
        };

        group.update_transformation(&transformation);

        if let Some(material) = material {
            group.replace_material(&material);
        }

        group.into()
    }
}
