use crate::{
    intersection::{Intersectable, ListBuilder},
    math::{Point, Ray, Vector},
};

/// A `Plane` is an infinitely large plane situated along the x and z axes.
#[derive(Clone, Copy, Debug)]
pub struct Plane;

impl Intersectable for Plane {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<ListBuilder<'a>> {
        todo!()
    }

    fn normal_at(&self, _point: &Point) -> Vector {
        Vector::y_axis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane;

        let n = Vector::y_axis();

        assert_approx_eq!(p.normal_at(&Point::origin()), n);
        assert_approx_eq!(p.normal_at(&Point::new(10.0, 0.0, -10.0)), n);
        assert_approx_eq!(p.normal_at(&Point::new(-5.0, 0.0, 150.0)), n);
    }
}
