use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Result};

use crate::math::Point;

#[derive(Debug)]
pub struct ObjParser {
    pub vertices: Vec<Point>,
    pub ignored: usize,
}

impl ObjParser {
    #[must_use]
    fn new() -> Self {
        Self { vertices: Vec::new(), ignored: 0 }
    }

    #[must_use]
    pub fn parse<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;

        let buffer = BufReader::new(file).lines();

        let mut parser = Self::new();

        for line in buffer {
            let line = line?;
            let line = line.trim();

            if line.starts_with('v') {
                parser.parse_vertex(line)?;
            } else {
                parser.ignored += 1;
            }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

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

        assert!(p.is_err());

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
}
