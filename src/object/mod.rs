mod bounding_box;
mod csg;
mod group;
mod includes;
mod obj_parser;
mod shape;
mod shapes;
mod updatable;

use std::path::Path;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use paste::paste;

use self::{
    bounding_box::{Bounded, BoundingBox},
    csg::Csg,
    group::{Group, GroupBuilder},
    includes::Includes,
    obj_parser::ObjParser,
    shape::{Shape, ShapeBuilder},
    shapes::Shapes,
};
pub use self::{csg::Operation, updatable::Updatable};
use crate::{
    intersection::{Intersection, List},
    math::{
        float::impl_approx_eq, Point, Ray, Transformable, Transformation,
        Vector,
    },
    Material,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Object {
    Csg(Csg),
    Group(Group),
    Shape(Shape),
}

macro_rules! add_builder_fn {
    ($shape:ident($($args:ident : $ty:ty $(,)?)*)) => {
        paste! {
            pub fn [<$shape:lower _builder>]($($args:$ty,)*) -> ShapeBuilder {
                Shape::builder().shape(
                    Shapes::[<new_ $shape:lower>]($($args,)*)
                )
            }
        }
    };
}

impl Object {
    add_builder_fn!(Cone(minimum: f64, maximum:f64, closed: bool));
    add_builder_fn!(Cube());
    add_builder_fn!(Cylinder(minimum: f64, maximum: f64, closed: bool));
    add_builder_fn!(Plane());
    add_builder_fn!(Sphere());
    #[cfg(test)]
    add_builder_fn!(Test());
    add_builder_fn!(Triangle(
        point1: Point,
        point2: Point,
        point3: Point,
        normal1: Vector,
        normal2: Vector,
        normal3: Vector,
    ));

    pub fn flat_triangle_builder(
        point1: Point,
        point2: Point,
        point3: Point,
    ) -> ShapeBuilder {
        Shape::builder()
            .shape(Shapes::new_flat_triangle(point1, point2, point3))
    }

    pub fn group_builder() -> GroupBuilder {
        Group::builder()
    }

    #[must_use]
    pub fn new_csg(operation: Operation, left: Self, right: Self) -> Self {
        Csg::new(operation, left, right).into()
    }

    /// Parse a given OBJ file and return a partially formed `Group` containing
    /// all the triangles from the OBJ file.
    ///
    /// # Errors
    ///
    /// Will return errors if unable to read or parse the file.
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<GroupBuilder> {
        Ok(ObjParser::parse(filename)?.into_group())
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        match self {
            Self::Csg(csg) => csg.intersect(ray),
            Self::Group(group) => group.intersect(ray),
            Self::Shape(shape) => shape.intersect(ray, self),
        }
    }

    #[must_use]
    pub fn normal_at(
        &self,
        point: &Point,
        intersection: &Intersection,
    ) -> Vector {
        match self {
            Self::Csg(_) | Self::Group(_) => unreachable!(),
            Self::Shape(shape) => shape.normal_at(point, intersection),
        }
    }

    #[must_use]
    pub fn material(&self) -> &Material {
        match self {
            Self::Csg(_) | Self::Group(_) => unreachable!(),
            Self::Shape(shape) => &shape.material,
        }
    }

    #[must_use]
    pub fn casts_shadow(&self) -> bool {
        match self {
            Self::Csg(_) | Self::Group(_) => unreachable!(),
            Self::Shape(shape) => shape.casts_shadow,
        }
    }

    #[must_use]
    pub fn to_object_space<T: Transformable>(&self, value: &T) -> T {
        match self {
            Self::Csg(_) | Self::Group(_) => unreachable!(),
            Self::Shape(shape) => shape.to_object_space(value),
        }
    }

    #[must_use]
    pub fn divide(self, threshold: u32) -> Self {
        match self {
            Self::Csg(csg) => Self::Csg(csg.divide(threshold)),
            Self::Group(group) => Self::Group(group.divide(threshold)),
            Self::Shape(_) => self,
        }
    }
}

impl_approx_eq!(
    enum Object {
        Csg,
        Group,
        Shape,
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn create_from_file() {
        let _ = Object::from_file("src/object/tests/triangles.obj")
            .unwrap()
            .build();
    }

    #[test]
    fn comparing_objects() {
        let o1 = Object::group_builder().build();
        let o2 = Object::group_builder().build();
        let o3 = Object::sphere_builder().build();

        assert_approx_eq!(o1, &o2);

        assert_approx_ne!(o1, &o3);
    }
}
