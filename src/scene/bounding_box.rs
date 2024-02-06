use std::f64::consts::PI;

use raytracer::{
    math::{Angle, Point, Transformation, Vector},
    Camera, Colour, Light, Material, Object, World,
};

use super::SceneData;
use crate::arguments::Arguments;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn generate_scene(arguments: &Arguments) -> SceneData {
    let horizontal_size = arguments.width.unwrap_or(1000);
    let vertical_size = arguments.height.unwrap_or(400);
    let field_of_view = arguments.fov.unwrap_or(Angle(1.2));

    let camera = Camera::new(
        horizontal_size,
        vertical_size,
        field_of_view,
        Transformation::view_transformation(
            &Point::new(0.0, 2.5, -10.0),
            &Point::new(0.0, 1.0, 0.0),
            &Vector::y_axis(),
        ),
    );

    let mut world = World::new();

    world.add_light(Light::new_point(
        Point::new(-10.0, 100.0, -100.0),
        Colour::white(),
    ));
    world.add_light(Light::new_point(
        Point::new(0.0, 100.0, 0.0),
        Colour::new(0.1, 0.1, 0.1),
    ));
    world.add_light(Light::new_point(
        Point::new(100.0, 10.0, -25.0),
        Colour::new(0.2, 0.2, 0.2),
    ));
    world.add_light(Light::new_point(
        Point::new(-100.0, 10.0, -25.0),
        Colour::new(0.2, 0.2, 0.2),
    ));

    let bounding_box = Object::group_builder()
        .add_object(
            Object::cube_builder()
                .casts_shadow(false)
                .transformation(
                    Transformation::new()
                        .translate(1.0, 1.0, 1.0)
                        .scale(3.733_35, 2.584_5, 1.628_3)
                        .translate(-3.986_3, -0.121_7, -1.182),
                )
                .build(),
        )
        .transformation(
            Transformation::new()
                .translate(0.0, 0.121_7, 0.0)
                .scale(0.268, 0.268, 0.268),
        );

    let dragon =
        Object::from_obj_file("obj/dragon.obj").unwrap().transformation(
            Transformation::new()
                .translate(0.0, 0.121_7, 0.0)
                .scale(0.268, 0.268, 0.268),
        );

    let pedestal = Object::cylinder_builder(-0.15, 0.0, true)
        .material(
            Material::builder()
                .pattern(Colour::new(0.2, 0.2, 0.2).into())
                .ambient(0.0)
                .diffuse(0.8)
                .specular(0.0)
                .reflective(0.2)
                .build(),
        )
        .build();

    let threshold = 50;

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal.clone(),
                Object::group_builder()
                    .set_objects(vec![
                        dragon
                            .clone()
                            .material(
                                Material::builder()
                                    .pattern(Colour::new(1.0, 0.0, 0.1).into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                        bounding_box
                            .clone()
                            .material(
                                Material::builder()
                                    .ambient(0.0)
                                    .diffuse(0.4)
                                    .specular(0.0)
                                    .transparency(0.6)
                                    .refractive_index(1.0)
                                    .build(),
                            )
                            .build(),
                    ])
                    .build(),
            ])
            .transformation(Transformation::new().translate(0.0, 2.0, 0.0))
            .build()
            .divide(threshold),
    );

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal.clone(),
                Object::group_builder()
                    .set_objects(vec![
                        dragon
                            .clone()
                            .material(
                                Material::builder()
                                    .pattern(Colour::new(1.0, 0.5, 0.1).into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                        bounding_box
                            .clone()
                            .material(
                                Material::builder()
                                    .ambient(0.0)
                                    .diffuse(0.2)
                                    .specular(0.0)
                                    .transparency(0.8)
                                    .refractive_index(1.0)
                                    .build(),
                            )
                            .build(),
                    ])
                    .transformation(
                        Transformation::new()
                            .rotate_y(Angle(4.0))
                            .scale(0.75, 0.75, 0.75),
                    )
                    .build(),
            ])
            .transformation(Transformation::new().translate(2.0, 1.0, -1.0))
            .build()
            .divide(threshold),
    );

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal.clone(),
                Object::group_builder()
                    .set_objects(vec![
                        dragon
                            .clone()
                            .material(
                                Material::builder()
                                    .pattern(Colour::new(0.9, 0.5, 0.1).into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                        bounding_box
                            .clone()
                            .material(
                                Material::builder()
                                    .ambient(0.0)
                                    .diffuse(0.2)
                                    .specular(0.0)
                                    .transparency(0.8)
                                    .refractive_index(1.0)
                                    .build(),
                            )
                            .build(),
                    ])
                    .transformation(
                        Transformation::new()
                            .rotate_y(Angle(-0.4))
                            .scale(0.75, 0.75, 0.75),
                    )
                    .build(),
            ])
            .transformation(Transformation::new().translate(-2.0, 0.75, -1.0))
            .build()
            .divide(threshold),
    );

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal.clone(),
                Object::group_builder()
                    .set_objects(vec![
                        dragon
                            .clone()
                            .material(
                                Material::builder()
                                    .pattern(Colour::new(1.0, 0.9, 0.1).into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                        bounding_box
                            .clone()
                            .material(
                                Material::builder()
                                    .ambient(0.0)
                                    .diffuse(0.1)
                                    .specular(0.0)
                                    .transparency(0.9)
                                    .refractive_index(1.0)
                                    .build(),
                            )
                            .build(),
                    ])
                    .transformation(
                        Transformation::new()
                            .rotate_y(Angle(-0.2))
                            .scale(0.5, 0.5, 0.5),
                    )
                    .build(),
            ])
            .transformation(Transformation::new().translate(-4.0, 0.0, -2.0))
            .build()
            .divide(threshold),
    );

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal.clone(),
                Object::group_builder()
                    .set_objects(vec![
                        dragon
                            .clone()
                            .material(
                                Material::builder()
                                    .pattern(Colour::new(0.9, 1.0, 0.1).into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                        bounding_box
                            .material(
                                Material::builder()
                                    .ambient(0.0)
                                    .diffuse(0.1)
                                    .specular(0.0)
                                    .transparency(0.9)
                                    .refractive_index(1.0)
                                    .build(),
                            )
                            .build(),
                    ])
                    .transformation(
                        Transformation::new()
                            .rotate_y(Angle(3.3))
                            .scale(0.5, 0.5, 0.5),
                    )
                    .build(),
            ])
            .transformation(Transformation::new().translate(4.0, 0.0, -2.0))
            .build()
            .divide(threshold),
    );

    world.add_object(
        Object::group_builder()
            .set_objects(vec![
                pedestal,
                Object::group_builder()
                    .add_object(
                        dragon
                            .material(
                                Material::builder()
                                    .pattern(Colour::white().into())
                                    .ambient(0.1)
                                    .diffuse(0.6)
                                    .specular(0.3)
                                    .shininess(15.0)
                                    .build(),
                            )
                            .build(),
                    )
                    .transformation(Transformation::new().rotate_y(Angle(PI)))
                    .build(),
            ])
            .transformation(Transformation::new().translate(0.0, 0.5, -4.0))
            .build()
            .divide(threshold),
    );

    SceneData::new(camera, world)
}
