use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

#[derive(Debug)]
pub struct ObjParser {
    pub ignored: usize,
}

impl ObjParser {
    #[must_use]
    pub fn parse<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = File::open(filename)?;

        let buffer = BufReader::new(file).lines();

        let mut ignored = 0;

        for _line in buffer {
            ignored += 1;
        }

        Ok(Self { ignored })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_parser() {
        let p = ObjParser::parse("obj/test/gibberish.obj").unwrap();

        assert_eq!(p.ignored, 4);
    }
}
