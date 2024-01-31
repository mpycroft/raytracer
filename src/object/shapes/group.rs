use derive_new::new;
use float_cmp::{ApproxEq, F64Margin};

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{Point, Ray, Vector},
    object::shapes::Shapes,
    Object,
};

/// A `Group` is a collection of `Object`s that can be treated as a single
/// entity.
#[derive(Clone, Debug, new)]
pub struct Group(Vec<Object>);

impl Group {
    #[must_use]
    pub const fn objects(&self) -> &Vec<Object> {
        &self.0
    }

    #[must_use]
    pub fn iter_no_groups(&mut self) -> Vec<&mut Object> {
        let mut objects: Vec<&mut Object> = Vec::new();

        for object in &mut self.0 {
            // There must be a nicer way to handle this in a single match block
            // or if let else statement. However no matter the construction I
            // can't seem to get it right, we aren't able to reborrow object to
            // add to objects because we are matching on &mut object.shape.
            if object.is_group() {
                objects.extend(object.iter_no_groups());
            } else {
                objects.push(object);
            }
        }

        objects
    }

    pub fn update_bounding_box(&mut self) {
        for object in &mut self.0 {
            let Object::Shape(shape) = object;
            if let Shapes::Group(group) = &mut shape.shape {
                group.update_bounding_box();

                shape.bounding_box = shape.bounding_box();
            };
        }
    }
}

impl Intersectable for Group {
    #[must_use]
    fn intersect<'a>(
        &self,
        _ray: &Ray,
        _object: &'a Object,
    ) -> Option<List<'a>> {
        unreachable!()
    }

    #[must_use]
    fn normal_at(
        &self,
        _point: &Point,
        _intersection: &Intersection,
    ) -> Vector {
        unreachable!()
    }
}

impl Bounded for Group {
    fn bounding_box(&self) -> BoundingBox {
        let mut bounding_box = BoundingBox::default();

        for object in self.objects() {
            let Object::Shape(shape) = object;
            bounding_box += shape.bounding_box();
        }

        bounding_box
    }
}

impl ApproxEq for &Group {
    type Margin = F64Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        let margin = margin.into();

        for (lhs, rhs) in self.0.iter().zip(&other.0) {
            if !lhs.approx_eq(rhs, margin) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{float::*, Transformation};

    #[test]
    fn iter_no_groups() {
        let s1 = Object::sphere_builder().build();
        let s2 = Object::sphere_builder()
            .transformation(Transformation::new().translate(1.0, 0.0, 0.0))
            .build();
        let s3 = Object::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
            .build();

        let mut o = Object::group_builder(vec![Object::group_builder(vec![
            Object::group_builder(vec![
                s2.clone(),
                Object::group_builder(vec![s3.clone()]).build(),
            ])
            .build(),
            s1.clone(),
        ])
        .build()])
        .build();

        let Object::Shape(s) = &mut o;
        let Shapes::Group(g) = &mut s.shape else { unreachable!() };

        let v = g.iter_no_groups();

        assert_eq!(v.len(), 3);
        assert_approx_eq!(v[0], &s2);
        assert_approx_eq!(v[1], &s3);
        assert_approx_eq!(v[2], &s1);
    }

    #[test]
    fn comparing_groups() {
        let g1 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g2 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
        ]);
        let g3 = Group::new(vec![
            Object::sphere_builder().build(),
            Object::plane_builder().build(),
            Object::plane_builder().build(),
        ]);

        assert_approx_eq!(g1, &g2);

        assert_approx_ne!(g1, &g3);
    }
}
