use std::f64::{INFINITY, NEG_INFINITY};

use anyhow::Result;
use paste::paste;
use serde::Deserialize;

use super::{Data, Material, TransformationList};
use crate::{math::Transformation, Object};

macro_rules! create_shape {
    ($name:ident { $($arg:ident: $ty:ty $(,)?)* }) => {
        #[derive(Clone, Debug, Deserialize)]
        pub struct $name {
            $($arg: $ty,)*
            transform: Option<TransformationList>,
            material: Option<Material>,
            shadow: Option<bool>,
        }
    };
}

create_shape!(Cone {
    min: Option<f64>,
    max: Option<f64>,
    closed: Option<bool>
});
create_shape!(Cube {});
create_shape!(Cylinder {
    min: Option<f64>,
    max: Option<f64>,
    closed: Option<bool>
});
create_shape!(Plane {});
create_shape!(Sphere {});

macro_rules! impl_parse {
    ($name:ident { $($arg:ident: $default:expr $(,)?)* }) => {
        impl $name {
            pub fn parse(self, data: &mut Data) -> Result<Object> {
                let transformation = self.transform.map_or_else(
                    || Ok(Transformation::new()),
                    |list| list.parse(data),
                )?;

                let material = self.material.map_or_else(
                    || Ok(crate::Material::default()),
                    |material| material.parse(data),
                )?;

                paste! {
                    Ok(Object::[<$name:lower _builder>](
                        $(self.$arg.unwrap_or($default),)*
                    )
                    .transformation(transformation)
                    .material(material)
                    .casts_shadow(self.shadow.unwrap_or(true))
                    .build())
                }
            }
        }
    };
}

impl_parse!(Cone { min: NEG_INFINITY, max: INFINITY, closed: false });
impl_parse!(Cube {});
impl_parse!(Cylinder { min: NEG_INFINITY, max: INFINITY, closed: false });
impl_parse!(Plane {});
impl_parse!(Sphere {});

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;
    use crate::{math::float::assert_approx_eq, Colour};

    #[test]
    fn parse_cone() {
        let c: Cone = from_str(
            "\
min: 0
closed: true
material:
    color: [0, 1, 0]
transform:
    - [translate, 1, 2, 3]",
        )
        .unwrap();

        let mut d = Data::new();

        let c = c.parse(&mut d).unwrap();
        assert_approx_eq!(
            c,
            &Object::cone_builder(0.0, INFINITY, true)
                .material(
                    crate::Material::builder()
                        .pattern(Colour::green().into())
                        .build()
                )
                .transformation(Transformation::new().translate(1.0, 2.0, 3.0))
                .build()
        );
    }

    #[test]
    fn parse_cube() {
        let c: Cube = from_str("").unwrap();

        let mut d = Data::new();

        let c = c.parse(&mut d).unwrap();
        assert_approx_eq!(c, &Object::cube_builder().build());
    }

    #[test]
    fn parse_cylinder() {
        let c: Cylinder = from_str(
            "\
min: -1
max: 5
material: foo",
        )
        .unwrap();

        let mut d = Data::new();
        d.materials
            .insert(String::from("foo"), from_str("color: [1, 0, 0]").unwrap());

        let c = c.parse(&mut d).unwrap();
        assert_approx_eq!(
            c,
            &Object::cylinder_builder(-1.0, 5.0, false)
                .material(
                    crate::Material::builder()
                        .pattern(Colour::red().into())
                        .build()
                )
                .build()
        );
    }

    #[test]
    fn parse_plane() {
        let c: Plane = from_str(
            "\
material:
    ambient: 1.0
transform:
    - foo",
        )
        .unwrap();

        let mut d = Data::new();
        d.transformations.insert(
            String::from("foo"),
            from_str("- [translate, 1, 1, 1]").unwrap(),
        );

        let c = c.parse(&mut d).unwrap();
        assert_approx_eq!(
            c,
            &Object::plane_builder()
                .material(crate::Material::builder().ambient(1.0).build())
                .transformation(Transformation::new().translate(1.0, 1.0, 1.0))
                .build()
        );
    }

    #[test]
    fn parse_sphere() {
        let c: Sphere = from_str("").unwrap();

        let mut d = Data::new();

        let c = c.parse(&mut d).unwrap();
        assert_approx_eq!(c, &Object::sphere_builder().build());
    }
}
