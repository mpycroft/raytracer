mod bounding_box;
mod group;
mod obj_parser;
mod shape;
mod shapes;

use std::path::Path;

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use float_cmp::{ApproxEq, F64Margin};
use group::Group;
use paste::paste;
use shape::{Shape, ShapeBuilder};
use shapes::Shapes;

use self::{
    bounding_box::{Bounded, BoundingBox},
    group::GroupBuilder,
    obj_parser::ObjParser,
};
use crate::{
    intersection::{Intersection, List},
    math::{Point, Ray, Transformable, Transformation, Vector},
    Material,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Debug)]
#[enum_dispatch]
pub enum Object {
    Shape(Shape),
    Group(Group),
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

    /// Parse a given OBJ file and return a partially formed `Group` containing
    /// all the triangles from the OBJ file.
    ///
    /// # Errors
    ///
    /// Will return errors if unable to read or parse the file.
    pub fn from_obj_file<P: AsRef<Path>>(filename: P) -> Result<GroupBuilder> {
        Ok(ObjParser::parse(filename)?.into_group())
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        match self {
            Self::Shape(shape) => shape.intersect(ray, self),
            Self::Group(group) => group.intersect(ray),
        }
    }

    #[must_use]
    pub fn normal_at(
        &self,
        point: &Point,
        intersection: &Intersection,
    ) -> Vector {
        match self {
            Self::Shape(shape) => shape.normal_at(point, intersection),
            Self::Group(_) => unreachable!(),
        }
    }

    #[must_use]
    pub fn material(&self) -> &Material {
        match self {
            Self::Shape(shape) => &shape.material,
            Self::Group(_) => unreachable!(),
        }
    }

    #[must_use]
    pub fn casts_shadow(&self) -> bool {
        match self {
            Self::Shape(shape) => shape.casts_shadow,
            Self::Group(_) => unreachable!(),
        }
    }

    #[must_use]
    pub fn to_object_space<T: Transformable>(&self, value: &T) -> T {
        match self {
            Self::Shape(shape) => shape.to_object_space(value),
            Self::Group(_) => unreachable!(),
        }
    }

    fn update_transformation(&mut self, transformation: &Transformation) {
        match self {
            Self::Shape(shape) => shape.update_transformation(transformation),
            Self::Group(group) => group.update_transformation(transformation),
        }
    }

    fn update_material(&mut self, material: &Material) {
        match self {
            Self::Shape(shape) => shape.update_material(material),
            Self::Group(group) => group.update_material(material),
        }
    }
}

impl ApproxEq for &Object {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let margin = margin.into();

        match (self, other) {
            (Object::Shape(lhs), Object::Shape(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (Object::Group(lhs), Object::Group(rhs)) => {
                lhs.approx_eq(rhs, margin)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn create_from_obj_file() {
        let _ =
            Object::from_obj_file("obj/test/triangles.obj").unwrap().build();
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
