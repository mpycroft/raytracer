use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Result};

use crate::{math::Point, Object};

#[derive(Debug)]
pub struct ObjParser {
    pub vertices: Vec<Point>,
    pub groups: Vec<Object>,
    pub ignored: usize,
}

impl ObjParser {
    #[must_use]
    fn new() -> Self {
        Self { vertices: Vec::new(), groups: Vec::new(), ignored: 0 }
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

        let mut groups = HashMap::from([("default", Vec::new())]);

        let current_group =
            groups.get_mut("default").unwrap_or_else(|| unreachable!());

        for line in buffer {
            let line = line?;
            let line = line.trim();

            if line.starts_with('v') {
                parser.parse_vertex(line)?;
            } else if line.starts_with('f') {
                parser.parse_face(line, current_group)?;
            } else {
                parser.ignored += 1;
            }
        }

        for (_, triangles) in groups {
            parser.groups.push(Object::group_builder(triangles).build());
        }

        Ok(parser)
    }

    fn parse_vertex(&mut self, line: &str) -> Result<()> {
        let items: Vec<&str> = line.split(' ').collect();

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

    fn parse_face(
        &mut self,
        line: &str,
        group: &mut Vec<Object>,
    ) -> Result<()> {
        let items: Vec<&str> = line.split(' ').collect();

        if items.len() < 4 {
            bail!(
                "\
Expected 'f' followed by at least 3 space separated numbers for a face.
Found {} items.",
                items.len()
            );
        }

        if items.len() == 4 {
            let vertex1 = items[1].parse::<usize>()? - 1;
            let vertex2 = items[2].parse::<usize>()? - 1;
            let vertex3 = items[3].parse::<usize>()? - 1;

            group.push(
                Object::triangle_builder(
                    self.vertices[vertex1],
                    self.vertices[vertex2],
                    self.vertices[vertex3],
                )
                .build(),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, shape::Shape};

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

        let Shape::Group(g) = &p.groups[0].shape else { unreachable!() };
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
}
