/// A Point is a representation of a geometric position within the 3 dimensional
/// scene we are working on
#[derive(Clone, Copy, Debug)]
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
    use crate::math::float::assert_approx_eq;

    #[test]
    fn creating_a_point() {
        let p = Point::new(4.3, -4.2, 3.1);

        assert_approx_eq!(p.x, 4.3);
        assert_approx_eq!(p.y, -4.2);
        assert_approx_eq!(p.z, 3.1);
    }
}
