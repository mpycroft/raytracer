use std::f64::{INFINITY, NEG_INFINITY};

use anyhow::{bail, Result};
use paste::paste;
use serde::Deserialize;
use serde_yaml::{from_value, to_value, Value};

use super::{Add, Data, HashValue, Material, TransformationList};
use crate::{math::Transformation, Object, Operation};

macro_rules! create_shape {
    ($name:ident { $($arg:ident: $ty:ty $(,)?)* }) => {
        paste! {
            #[doc = "A `" $name "` holds deserialized object data"]
            #[derive(Clone, Debug, Deserialize)]
            pub struct $name {
                $($arg: $ty,)*
                transform: Option<TransformationList>,
                material: Option<Material>,
                shadow: Option<bool>,
            }
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
create_shape!(Group { children: Vec<Add>, divide: Option<u32> });
create_shape!(Plane {});
create_shape!(Sphere {});

/// `CsgShape` is a helper type since the Yaml definition uses a different tag
/// than when adding objects, and this saves us converting to from a `HashMap`
/// just to access the tag.
#[derive(Clone, Debug, Deserialize)]
struct CsgShape {
    #[serde(rename = "type")]
    tag: String,
    #[serde(flatten)]
    value: Value,
}

/// A `Csg` holds deserialized object data.
#[derive(Clone, Debug, Deserialize)]
struct Csg {
    operation: Operation,
    left: CsgShape,
    right: CsgShape,
}

fn get_transform(
    transformations: Option<TransformationList>,
    data: &Data,
) -> Result<crate::math::Transformation> {
    transformations
        .map_or_else(|| Ok(Transformation::new()), |list| list.parse(data))
}

fn get_material(
    material: Option<Material>,
    data: &Data,
) -> Result<crate::Material> {
    material.map_or_else(
        || Ok(crate::Material::default()),
        |material| material.parse(data),
    )
}

macro_rules! impl_parse {
    ($name:ident { $($arg:ident: $default:expr $(,)?)* }) => {
        impl $name {
            pub fn parse(self, data: &Data) -> Result<Object> {
                let transformation = get_transform(self.transform, data)?;
                let material = get_material(self.material, data)?;

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

impl Group {
    pub fn parse(self, data: &Data) -> Result<Object> {
        let mut objects = Vec::new();

        for object in self.children {
            objects.push(parse_shape(&object.add, object.value, data)?);
        }

        /// Due to the typed nature of `TypedBuilder` we cannot easily
        /// conditionally set values e.g. .transformation but not .material
        /// because the return types from an if will be different. This is ugly
        /// but we only need to do it for groups.
        macro_rules! group_builder {
            (@shadow $self:ident; ($expr:expr)) => {
                if let Some(shadow) = $self.shadow {
                    $expr.casts_shadow(shadow).build()
                } else {
                    $expr.build()
                }
            };
            (@material $self:ident; ($expr:expr)) => {
                if $self.material.is_some() {
                    let material = get_material($self.material, data)?;

                    group_builder!(
                        @shadow $self; ($expr.material(material))
                    )
                } else {
                    group_builder!(@shadow $self; ($expr))
                }
            };
            (@transform $self:ident; ($expr:expr)) => {
                if self.transform.is_some() {
                    let transformation = get_transform($self.transform, data)?;
                    group_builder!(
                        @material $self; ($expr.transformation(transformation))
                    )
                } else {
                    group_builder!(@material $self; ($expr))
                }
            };
            ($group:ident, $self:ident) => {
                group_builder!(@transform $self; ($group))
            };
        }

        let group = Object::group_builder().set_objects(objects);

        let group = group_builder!(group, self);

        let group = if let Some(divide) = self.divide {
            group.divide(divide)
        } else {
            group
        };

        Ok(group)
    }
}

impl Csg {
    pub fn parse(self, data: &Data) -> Result<Object> {
        Ok(Object::new_csg(
            self.operation,
            parse_shape(&self.left.tag, self.left.value, data)?,
            parse_shape(&self.right.tag, self.right.value, data)?,
        ))
    }
}

pub fn parse_shape(tag: &str, value: Value, data: &Data) -> Result<Object> {
    macro_rules! map_to_object {
        ($name:literal) => {{
            paste! {
                from_value::<[<$name:camel>]>(value)?.parse(data)
            }
        }};
    }

    match tag {
        "cone" => map_to_object!("cone"),
        "csg" => map_to_object!("csg"),
        "cube" => map_to_object!("cube"),
        "cylinder" => map_to_object!("cylinder"),
        "group" => map_to_object!("group"),
        "plane" => map_to_object!("plane"),
        "sphere" => map_to_object!("sphere"),
        _ => {
            if let Some(define) = data.shapes.get(tag) {
                let mut shape: HashValue = from_value(value)?;

                let define = define.clone();
                let mut define_values: HashValue = from_value(define.value)?;

                if let Some(mut transform) = shape.remove("transform") {
                    if let Some(define_transform) =
                        define_values.remove("transform")
                    {
                        transform = TransformationList::combine(
                            define_transform,
                            transform,
                        )?;
                    };

                    define_values.insert(String::from("transform"), transform);
                }

                if let Some(material) = shape.remove("material") {
                    define_values.insert(String::from("material"), material);
                }

                if let Some(shadow) = shape.remove("shadow") {
                    define_values.insert(String::from("shadow"), shadow);
                }

                Ok(parse_shape(&define.add, to_value(define_values)?, data)?)
            } else {
                bail!("Reference to shape '{tag}' that was not defined")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::from_str;

    use super::*;
    use crate::{
        math::{float::*, Transformation},
        Colour,
    };

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

        let d = Data::new();

        let o = c.parse(&d).unwrap();
        assert_approx_eq!(
            o,
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

        let d = Data::new();

        let o = c.parse(&d).unwrap();
        assert_approx_eq!(o, &Object::cube_builder().build());
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

        let o = c.parse(&d).unwrap();
        assert_approx_eq!(
            o,
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
    fn parse_group() {
        let g: Group = from_str(
            "\
children:
    - add: sphere
      transform:
          - [translate, -2, -2, 0]
      shadow: false
    - add: sphere
      transform:
          - [translate, -2, 2, 0]
      shadow: true
    - add: sphere
      transform:
          - [scale, 4, 4, 4]
      material:
          diffuse: 0
      shadow: false
divide: 1",
        )
        .unwrap();

        let d = Data::new();

        let o = g.parse(&d).unwrap();

        assert_approx_eq!(
            o,
            &Object::group_builder()
                .set_objects(vec![
                    Object::sphere_builder()
                        .transformation(
                            Transformation::new().translate(-2.0, -2.0, 0.0)
                        )
                        .casts_shadow(false)
                        .build(),
                    Object::sphere_builder()
                        .transformation(
                            Transformation::new().translate(-2.0, 2.0, 0.0)
                        )
                        .casts_shadow(true)
                        .build(),
                    Object::sphere_builder()
                        .transformation(
                            Transformation::new().scale(4.0, 4.0, 4.0)
                        )
                        .material(
                            crate::Material::builder().diffuse(0.0).build()
                        )
                        .casts_shadow(false)
                        .build(),
                ])
                .build()
                .divide(1)
        );
    }

    #[test]
    fn parse_plane() {
        let p: Plane = from_str(
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

        let o = p.parse(&d).unwrap();
        assert_approx_eq!(
            o,
            &Object::plane_builder()
                .material(crate::Material::builder().ambient(1.0).build())
                .transformation(Transformation::new().translate(1.0, 1.0, 1.0))
                .build()
        );
    }

    #[test]
    fn parse_sphere() {
        let s: Sphere = from_str("").unwrap();

        let d = Data::new();

        let o = s.parse(&d).unwrap();
        assert_approx_eq!(o, &Object::sphere_builder().build());
    }

    #[test]
    fn parse_csg() {
        let c: Csg = from_str(
            "\
operation: difference
left:
    type: cube
    transform:
        - [scale, 2, 2, 2]
right:
    type: sphere
    material:
        color: [1, 0, 1]",
        )
        .unwrap();

        let d = Data::new();

        let o = c.parse(&d).unwrap();

        assert_approx_eq!(
            o,
            &Object::new_csg(
                Operation::Difference,
                Object::cube_builder()
                    .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                    .build(),
                Object::sphere_builder()
                    .material(
                        crate::Material::builder()
                            .pattern(Colour::purple().into())
                            .build()
                    )
                    .build()
            )
        );
    }
}
