use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Result};

use crate::{
    math::{Point, Vector},
    object::{shapes::Shapes, ObjectBuilder},
    Object,
};

#[derive(Debug)]
pub struct ObjParser {
    pub vertices: Vec<Point>,
    pub normals: Vec<Vector>,
    pub groups: Vec<Object>,
    pub ignored: usize,
}

impl ObjParser {
    #[must_use]
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            groups: Vec::new(),
            ignored: 0,
        }
    }

    /// Parse a given OBJ file.
    ///
    /// # Errors
    ///
    /// Will return errors if unable to read or parse the file.
    pub fn parse<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;

        let buffer = BufReader::new(file).lines();

        let mut parser = Self::new();

        let mut groups = HashMap::from([(String::from("default"), Vec::new())]);

        let mut current_group =
            groups.get_mut("default").unwrap_or_else(|| unreachable!());

        for line in buffer {
            let line = line?;
            let line = line.trim();

            if line.starts_with("v ") {
                parser.parse_vertex(line)?;
            } else if line.starts_with("vn ") {
                parser.parse_normal(line)?;
            } else if line.starts_with("f ") {
                parser.parse_face(line, current_group)?;
            } else if line.starts_with("g ") {
                current_group = Self::parse_group(line, &mut groups)?;
            } else {
                parser.ignored += 1;
            }
        }

        let mut groups = groups.into_iter().collect::<Vec<_>>();
        groups.sort_by(|a, b| a.0.cmp(&b.0));

        for (_, triangles) in groups {
            if !triangles.is_empty() {
                parser.groups.push(Object::group_builder(triangles).build());
            }
        }

        Ok(parser)
    }

    fn split(line: &str) -> Vec<&str> {
        line.split(' ').filter(|&s| !s.is_empty()).collect()
    }

    fn split_face(item: &str) -> Result<Vec<&str>> {
        let values: Vec<&str> = item.split('/').collect();

        if values.len() != 1 && values.len() != 3 {
            bail!(
                "\
Expected face values to be either 'num' or 'num//num' or 'num/num/num'
Found {}.",
                item
            )
        }

        Ok(values)
    }

    fn parse_vertex(&mut self, line: &str) -> Result<()> {
        let items = Self::split(line);

        if items.len() != 4 {
            bail!(
                "\
Expected 'v' followed by 3 space separated numbers for a vertex.
Found {} items.",
                items.len()
            );
        }

        let x = items[1].parse()?;
        let y = items[2].parse()?;
        let z = items[3].parse()?;

        self.vertices.push(Point::new(x, y, z));

        Ok(())
    }

    fn parse_normal(&mut self, line: &str) -> Result<()> {
        let items = Self::split(line);

        if items.len() != 4 {
            bail!(
                "\
Expected 'vn' followed by 3 space separated numbers for a normal.
Found {} items.",
                items.len()
            );
        }

        let x = items[1].parse()?;
        let y = items[2].parse()?;
        let z = items[3].parse()?;

        self.normals.push(Vector::new(x, y, z));

        Ok(())
    }

    fn parse_face(
        &mut self,
        line: &str,
        group: &mut Vec<Object>,
    ) -> Result<()> {
        let items = Self::split(line);

        if items.len() < 4 {
            bail!(
                "\
Expected 'f' followed by at least 3 space separated numbers for a face.
Found {} items.",
                items.len()
            );
        }

        let get_vertex_normal = |item: &str| -> Result<(usize, Option<usize>)> {
            let values = Self::split_face(item)?;

            let vertex = values[0].parse::<usize>()? - 1;
            let normal = if values.len() == 1 {
                None
            } else {
                Some(values[2].parse::<usize>()? - 1)
            };

            Ok((vertex, normal))
        };

        let (vertex1, normal1) = get_vertex_normal(items[1])?;

        for index in 2..(items.len() - 1) {
            let (vertex2, normal2) = get_vertex_normal(items[index])?;
            let (vertex3, normal3) = get_vertex_normal(items[index + 1])?;

            let is_smooth = if normal1.is_none() {
                false
            } else {
                if normal2.is_none() || normal3.is_none() {
                    bail!(
                        "\
If one vertex normal is specified, all faces must also provide vertex normals."
                    )
                }

                true
            };

            if is_smooth {
                group.push(
                    Object::smooth_triangle_builder(
                        self.vertices[vertex1],
                        self.vertices[vertex2],
                        self.vertices[vertex3],
                        // We have already checked these are all Some().
                        self.normals[normal1.unwrap()],
                        self.normals[normal2.unwrap()],
                        self.normals[normal3.unwrap()],
                    )
                    .build(),
                );
            } else {
                group.push(
                    Object::triangle_builder(
                        self.vertices[vertex1],
                        self.vertices[vertex2],
                        self.vertices[vertex3],
                    )
                    .build(),
                );
            }
        }

        Ok(())
    }

    fn parse_group<'a>(
        line: &str,
        groups: &'a mut HashMap<String, Vec<Object>>,
    ) -> Result<&'a mut Vec<Object>> {
        let group_name = line[1..].trim();

        if groups.insert(String::from(group_name), Vec::new()).is_some() {
            bail!("Group {group_name} is repeated.");
        }

        groups.get_mut(group_name).ok_or_else(|| unreachable!())
    }

    pub fn into_group(self) -> ObjectBuilder<((), (), (), (Shapes,))> {
        Object::group_builder(self.groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{float::*, Vector};

    #[test]
    fn ignoring_unrecognised_lines() {
        let p = ObjParser::parse("obj/test/gibberish.obj").unwrap();

        assert_eq!(p.ignored, 4);
    }

    #[test]
    fn parsing_vertices() {
        let p = ObjParser::parse("obj/test/vertices.obj").unwrap();

        assert_eq!(p.vertices.len(), 4);

        assert_approx_eq!(p.vertices[0], Point::new(-1.0, 1.0, 0.0));
        assert_approx_eq!(p.vertices[1], Point::new(-1.0, 0.5, 0.0));
        assert_approx_eq!(p.vertices[2], Point::new(1.0, 0.0, 0.0));
        assert_approx_eq!(p.vertices[3], Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn parsing_invalid_vertices() {
        let p = ObjParser::parse("obj/test/too_many_vertices.obj");

        let e = p.unwrap_err();

        assert_eq!(
            e.to_string(),
            "\
Expected 'v' followed by 3 space separated numbers for a vertex.
Found 5 items."
        );

        let p = ObjParser::parse("obj/test/invalid_vertices.obj");

        assert!(p.is_err());

        let e = p.unwrap_err();

        assert_eq!(e.to_string(), "invalid float literal");
    }

    #[test]
    fn parsing_faces() {
        let p = ObjParser::parse("obj/test/faces.obj").unwrap();

        let Shapes::Group(g) = &p.groups[0].shape else { unreachable!() };
        let c = g.objects();

        assert_eq!(c.len(), 2);

        assert_approx_eq!(
            c[0],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0)
            )
            .build()
        );
        assert_approx_eq!(
            c[1],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0)
            )
            .build()
        );
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 2 but the index is 2"
    )]
    fn parsing_invalid_faces() {
        let p = ObjParser::parse("obj/test/not_enough_faces.obj");

        let e = p.unwrap_err();

        assert_eq!(
            e.to_string(),
            "\
Expected 'f' followed by at least 3 space separated numbers for a face.
Found 3 items."
        );

        let _ = ObjParser::parse("obj/test/invalid_faces.obj");
    }

    #[test]
    fn triangulating_polygons() {
        let p = ObjParser::parse("obj/test/triangulating.obj").unwrap();

        let Shapes::Group(g) = &p.groups[0].shape else { unreachable!() };
        let c = g.objects();

        assert_eq!(c.len(), 3);

        assert_approx_eq!(
            c[0],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0)
            )
            .build()
        );
        assert_approx_eq!(
            c[1],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0)
            )
            .build()
        );
        assert_approx_eq!(
            c[2],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(0.0, 2.0, 0.0)
            )
            .build()
        );
    }

    #[test]
    fn triangles_in_groups() {
        let o = ObjParser::parse("obj/test/triangles.obj")
            .unwrap()
            .into_group()
            .build();

        let Shapes::Group(g) = &o.shape else { unreachable!() };
        let c = g.objects();

        assert_eq!(c.len(), 2);

        let Shapes::Group(g) = &c[0].shape else { unreachable!() };
        let c1 = g.objects();

        assert_eq!(c1.len(), 1);

        assert_approx_eq!(
            c1[0],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0)
            )
            .build()
        );

        let Shapes::Group(g) = &c[1].shape else { unreachable!() };
        let c2 = g.objects();

        assert_eq!(c2.len(), 1);

        assert_approx_eq!(
            c2[0],
            &Object::triangle_builder(
                Point::new(-1.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0)
            )
            .build()
        );
    }

    #[test]
    fn invalid_groups() {
        let p = ObjParser::parse("obj/test/invalid_groups.obj");

        let e = p.unwrap_err();

        assert_eq!(e.to_string(), "Group FirstGroup is repeated.");
    }

    #[test]
    fn parse_vertex_normal() {
        let p = ObjParser::parse("obj/test/normals.obj").unwrap();

        assert_eq!(p.normals.len(), 3);

        assert_approx_eq!(p.normals[0], Vector::z_axis());
        assert_approx_eq!(p.normals[1], Vector::new(0.707, 0.0, 0.707));
        assert_approx_eq!(p.normals[2], Vector::new(1.0, 2.0, -3.0));
    }

    #[test]
    fn parsing_invalid_normals() {
        let p = ObjParser::parse("obj/test/too_many_normals.obj");

        let e = p.unwrap_err();

        assert_eq!(
            e.to_string(),
            "\
Expected 'vn' followed by 3 space separated numbers for a normal.
Found 6 items."
        );

        let p = ObjParser::parse("obj/test/invalid_normals.obj");

        assert!(p.is_err());

        let e = p.unwrap_err();

        assert_eq!(e.to_string(), "invalid float literal");
    }

    #[test]
    fn parsing_face_normals() {
        let p = ObjParser::parse("obj/test/face_normals.obj").unwrap();

        let Shapes::Group(g) = &p.groups[0].shape else { unreachable!() };
        let c = g.objects();

        assert_eq!(c.len(), 2);

        let t = Object::smooth_triangle_builder(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::y_axis(),
            -Vector::x_axis(),
            Vector::x_axis(),
        )
        .build();

        assert_approx_eq!(c[0], &t);
        assert_approx_eq!(c[1], &t);
    }

    #[test]
    #[should_panic(
        expected = "index out of bounds: the len is 2 but the index is 2"
    )]
    fn parsing_invalid_face_normals() {
        let p = ObjParser::parse("obj/test/inconsistent_face_normals.obj");

        let e = p.unwrap_err();

        assert_eq!(
            e.to_string(),
            "\
If one vertex normal is specified, all faces must also provide vertex normals."
        );

        let p = ObjParser::parse("obj/test/invalid_face_normals.obj");

        let e = p.unwrap_err();

        assert_eq!(
            e.to_string(),
            "\
Expected face values to be either 'num' or 'num//num' or 'num/num/num'
Found 2///3."
        );

        let _ = ObjParser::parse("obj/test/invalid_index_face_normals.obj");
    }
}
