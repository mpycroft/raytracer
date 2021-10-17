use crate::{intersect::Intersectable, math::Ray};

/// A Sphere is a unit sphere centred at the origin (0, 0, 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Self {
        Self
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        todo!()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _ = Sphere::new();
    }
}
