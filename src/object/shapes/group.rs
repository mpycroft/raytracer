use derive_new::new;
use float_cmp::{ApproxEq, F64Margin};

use super::Intersectable;
use crate::{
    bounding_box::{Bounded, BoundingBox},
    intersection::{Intersection, List},
    math::{Point, Ray, Vector},
    object::shapes::Shapes,
    Shape,
};

/// A `Group` is a collection of `Object`s that can be treated as a single
/// entity.
#[derive(Clone, Debug, new)]
pub struct Group(Vec<Shape>);

impl Group {
    #[must_use]
    pub const fn objects(&self) -> &Vec<Shape> {
        &self.0
    }

    #[must_use]
    pub fn iter_no_groups(&mut self) -> Vec<&mut Shape> {
        let mut objects: Vec<&mut Shape> = Vec::new();

        for object in &mut self.0 {
            // There must be a nicer way to handle this in a single match block
            // or if let else statement. However no matter the construction I
            // can't seem to get it right, we aren't able to reborrow object to
            // add to objects because we are matching on &mut object.shape.
            if matches!(object.shape, Shapes::Group(_)) {
                let Shapes::Group(group) = &mut object.shape else {
                    unreachable!()
                };

                objects.extend(group.iter_no_groups());
            } else {
                objects.push(object);
            }
        }

        objects
    }

    pub fn update_bounding_box(&mut self) {
        for object in &mut self.0 {
            if let Shapes::Group(group) = &mut object.shape {
                group.update_bounding_box();

                object.bounding_box = object.bounding_box();
            };
        }
    }
}

impl Intersectable for Group {
    #[must_use]
    fn intersect<'a>(
        &self,
        _ray: &Ray,
        _object: &'a Shape,
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
            bounding_box += object.bounding_box();
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
        let s1 = Shape::sphere_builder().build();
        let s2 = Shape::sphere_builder()
            .transformation(Transformation::new().translate(1.0, 0.0, 0.0))
            .build();
        let s3 = Shape::sphere_builder()
            .transformation(Transformation::new().translate(0.0, 1.0, 0.0))
            .build();

        let mut o = Shape::group_builder(vec![Shape::group_builder(vec![
            Shape::group_builder(vec![
                s2.clone(),
                Shape::group_builder(vec![s3.clone()]).build(),
            ])
            .build(),
            s1.clone(),
        ])
        .build()])
        .build();

        let Shapes::Group(g) = &mut o.shape else { unreachable!() };

        let v = g.iter_no_groups();

        assert_eq!(v.len(), 3);
        assert_approx_eq!(v[0], &s2);
        assert_approx_eq!(v[1], &s3);
        assert_approx_eq!(v[2], &s1);
    }

    #[test]
    fn comparing_groups() {
        let g1 = Group::new(vec![
            Shape::sphere_builder().build(),
            Shape::plane_builder().build(),
        ]);
        let g2 = Group::new(vec![
            Shape::sphere_builder().build(),
            Shape::plane_builder().build(),
        ]);
        let g3 = Group::new(vec![
            Shape::sphere_builder().build(),
            Shape::plane_builder().build(),
            Shape::plane_builder().build(),
        ]);

        assert_approx_eq!(g1, &g2);

        assert_approx_ne!(g1, &g3);
    }
}
