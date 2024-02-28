use std::f64::{INFINITY, NEG_INFINITY};

use anyhow::{bail, Result};
use paste::paste;
use rand::prelude::*;
use serde::Deserialize;
use serde_yaml::{from_value, to_value, Value};

use super::{Add, Data, HashValue, Material, TransformationList};
use crate::{Object, Operation};

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
create_shape!(Obj { file: String, divide: Option<u32>});
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

/// Due to the typed nature of `TypedBuilder` we cannot easily conditionally set
/// values e.g. .transformation but not .material because the return types from
/// an if will be different. This is ugly but short of repeating ourselves with
/// nested if's there does not appear to be a nice way to handle this.
macro_rules! build_object {
    (@shadow $self:ident; ($expr:expr)) => {
        if let Some(shadow) = $self.shadow {
            $expr.casts_shadow(shadow).build()
        } else {
            $expr.build()
        }
    };
    (@transform $self:ident, $data:ident; ($expr:expr)) => {
        if let Some(transform) = $self.transform {
            let transformation = transform.parse($data)?;

            build_object!(
                @shadow $self; ($expr.transformation(transformation))
            )
        } else {
            build_object!(@shadow $self; ($expr))
        }
    };
    (@material $self:ident, $data:ident, $rng:ident; ($expr:expr)) => {
        if let Some(material) = $self.material {
            let material = material.parse($data, $rng)?;

            build_object!(@transform $self, $data; ($expr.material(material)))
        } else {
            build_object!(@transform $self, $data; ($expr))
        }
    };
    ($object:ident, $self:ident, $data:ident, $rng:ident) => {{
        build_object!(@material $self, $data, $rng; ($object))
    }};
}

macro_rules! impl_parse {
    ($name:ident { $($arg:ident: $default:expr $(,)?)* }) => {
        impl $name {
            pub fn parse<R: Rng>(
                self,
                data: &Data,
                rng: &mut R
            ) -> Result<Object> {
                paste! {
                    let object = Object::[<$name:lower _builder>](
                        $(self.$arg.unwrap_or($default),)*
                    );

                    Ok(build_object!(object, self, data, rng))
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
    pub fn parse<R: Rng>(self, data: &Data, rng: &mut R) -> Result<Object> {
        let mut objects = Vec::new();

        for object in self.children {
            objects.push(parse_shape(&object.add, object.value, data, rng)?);
        }

        let group = Object::group_builder().set_objects(objects);

        let mut object = build_object!(group, self, data, rng);

        if let Some(divide) = self.divide {
            object = object.divide(divide);
        };

        Ok(object)
    }
}

impl Obj {
    pub fn parse<R: Rng>(self, data: &Data, rng: &mut R) -> Result<Object> {
        let group = Object::from_file(self.file)?;

        let mut object = build_object!(group, self, data, rng);

        if let Some(divide) = self.divide {
            object = object.divide(divide);
        };

        Ok(object)
    }
}

impl Csg {
    pub fn parse<R: Rng>(self, data: &Data, rng: &mut R) -> Result<Object> {
        Ok(Object::new_csg(
            self.operation,
            parse_shape(&self.left.tag, self.left.value, data, rng)?,
            parse_shape(&self.right.tag, self.right.value, data, rng)?,
        ))
    }
}

pub fn parse_shape<R: Rng>(
    tag: &str,
    value: Value,
    data: &Data,
    rng: &mut R,
) -> Result<Object> {
    macro_rules! map_to_object {
        ($name:literal) => {{
            paste! {
                from_value::<[<$name:camel>]>(value)?.parse(data, rng)
            }
        }};
    }

    match tag {
        "cone" => map_to_object!("cone"),
        "csg" => map_to_object!("csg"),
        "cube" => map_to_object!("cube"),
        "cylinder" => map_to_object!("cylinder"),
        "group" => map_to_object!("group"),
        "obj" => map_to_object!("obj"),
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

                Ok(parse_shape(
                    &define.add,
                    to_value(define_values)?,
                    data,
                    rng,
                )?)
            } else {
                bail!("Reference to shape '{tag}' that was not defined")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand_xoshiro::Xoshiro256PlusPlus;
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

        let o = c.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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

        let o = c.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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

        let o = c.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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

        let o = g.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();

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
    fn parse_obj() {
        let o: Obj = from_str(
            "\
add: obj
file: src/scene/tests/dodecahedron.obj
transform:
    - [scale, 2, 2, 2]
material:
    color: [0, 1, 0]
shadow: false
divide: 1",
        )
        .unwrap();

        let d = Data::new();

        let o = o.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();

        assert_approx_eq!(
            o,
            &Object::from_file("src/scene/tests/dodecahedron.obj")
                .unwrap()
                .transformation(Transformation::new().scale(2.0, 2.0, 2.0))
                .material(
                    crate::Material::builder()
                        .pattern(Colour::green().into())
                        .build()
                )
                .casts_shadow(false)
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

        let o = p.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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

        let o = s.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();
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

        let o = c.parse(&d, &mut Xoshiro256PlusPlus::seed_from_u64(0)).unwrap();

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

    #[test]
    fn parse_defined_shape() {
        let v: Value = from_str(
            "\
transform:
    - [scale, 2, 2, 2]
material:
    color: [0, 0, 1]
shadow: false",
        )
        .unwrap();

        let mut d = Data::new();
        d.shapes.insert(
            String::from("foo"),
            from_str::<Add>(
                "\
add: sphere
transform:
    - [translate, 1, 2, 3]
material:
    color: [1, 0, 0]
shadow: true",
            )
            .unwrap(),
        );

        let o = parse_shape(
            "foo",
            v,
            &d,
            &mut Xoshiro256PlusPlus::seed_from_u64(0),
        )
        .unwrap();

        assert_approx_eq!(
            o,
            &Object::sphere_builder()
                .transformation(
                    Transformation::new()
                        .translate(1.0, 2.0, 3.0)
                        .scale(2.0, 2.0, 2.0)
                )
                .material(
                    crate::Material::builder()
                        .pattern(Colour::blue().into())
                        .build()
                )
                .casts_shadow(false)
                .build()
        );
    }
}
