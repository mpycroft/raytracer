//! A `Cylinder` is an infinite cylinder of radius 1 centred on the y axis.

use crate::{
    intersection::TList,
    math::{float::approx_eq, Point, Ray, Vector},
};

#[must_use]
pub fn intersect(ray: &Ray) -> Option<TList> {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

    if approx_eq!(a, 0.0) {
        return None;
    };

    let b =
        2.0 * (ray.origin.x * ray.direction.x + ray.origin.z * ray.direction.z);

    let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    };

    todo!()
}

#[must_use]
pub fn normal_at(point: &Point) -> Vector {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::float::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        assert!(intersect(&Ray::new(
            Point::new(1.0, 0.0, 0.0),
            Vector::y_axis()
        ))
        .is_none());
        assert!(
            intersect(&Ray::new(Point::origin(), Vector::y_axis())).is_none()
        );
        assert!(intersect(&Ray::new(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(1.0, 1.0, 1.0).normalise()
        ))
        .is_none());
    }
}
