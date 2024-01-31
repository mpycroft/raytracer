mod shape;
pub mod shapes;

use paste::paste;
pub use shape::{Shape, ShapeBuilder};
use shapes::Shapes;

use crate::math::{Point, Vector};

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
    add_builder_fn!(Group(objects: Vec<Shape>));
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
}
