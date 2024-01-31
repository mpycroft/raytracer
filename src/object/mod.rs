mod shape;
pub mod shapes;

use float_cmp::{ApproxEq, F64Margin};
use paste::paste;
pub use shape::{Shape, ShapeBuilder};
use shapes::Shapes;

use crate::{
    intersection::{Intersection, List},
    math::{Point, Ray, Transformable, Vector},
    Material,
};

/// An 'Object' represents some entity in the scene that can be rendered.
#[derive(Clone, Debug)]
pub enum Object {
    Shape(Shape),
}

macro_rules! add_builder_fn {
    ($shape:ident($($args:ident : $ty:ty $(,)?)*)) => {
        paste! {
            pub fn [<$shape:lower _builder>]($($args:$ty,)*) ->
                ShapeBuilder<((), (), (), (Shapes,))>
            {
                Shape::_builder().shape(
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
    add_builder_fn!(Group(objects: Vec<Object>));
    add_builder_fn!(Plane());
    add_builder_fn!(Sphere());
    #[cfg(test)]
    add_builder_fn!(Test());
    add_builder_fn!(Triangle(point1: Point, point2: Point, point3: Point));

    pub fn smooth_triangle_builder(
        point1: Point,
        point2: Point,
        point3: Point,
        normal1: Vector,
        normal2: Vector,
        normal3: Vector,
    ) -> ShapeBuilder<((), (), (), (Shapes,))> {
        Shape::_builder().shape(Shapes::new_smooth_triangle(
            point1, point2, point3, normal1, normal2, normal3,
        ))
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Option<List> {
        match self {
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
            Self::Shape(shape) => shape.normal_at(point, intersection),
        }
    }

    #[must_use]
    pub fn material(&self) -> &Material {
        match self {
            Self::Shape(shape) => &shape.material,
        }
    }

    #[must_use]
    pub fn casts_shadow(&self) -> bool {
        match self {
            Self::Shape(shape) => shape.casts_shadow,
        }
    }

    #[must_use]
    pub fn to_object_space<T: Transformable>(&self, value: &T) -> T {
        match self {
            Self::Shape(shape) => shape.to_object_space(value),
        }
    }

    #[must_use]
    pub fn to_world_space<T: Transformable>(&self, value: &T) -> T {
        match self {
            Self::Shape(shape) => shape.to_world_space(value),
        }
    }

    #[must_use]
    pub fn is_group(&self) -> bool {
        match self {
            Self::Shape(shape) => matches!(shape.shape, Shapes::Group(_)),
        }
    }

    #[must_use]
    pub fn iter_no_groups(&mut self) -> Vec<&mut Object> {
        match self {
            Self::Shape(shape) => match &mut shape.shape {
                Shapes::Group(group) => group.iter_no_groups(),
                _ => unreachable!(),
            },
        }
    }
}

impl From<Shape> for Object {
    fn from(value: Shape) -> Self {
        Self::Shape(value)
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
        }
    }
}
