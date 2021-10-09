/// A Point is a representation of a geometric position within the 3 dimensional
/// scene we are working on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert!((p.x - 4.3).abs() < f64::EPSILON);
        assert!((p.y - -4.2).abs() < f64::EPSILON);
        assert!((p.z - 3.1).abs() < f64::EPSILON);
    }
}
