use super::{util::impl_pattern, PatternAt};
use crate::{math::Point, Colour};

impl_pattern!(
    /// A `Blend` pattern averages the `Colour`s of two `Pattern`s.
    Blend
);

impl PatternAt for Blend {
    fn pattern_at(&self, point: &Point) -> Colour {
        (self.a.sub_pattern_at(point) + self.b.sub_pattern_at(point)) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::float::*, pattern::util::add_kind_tests};

    #[test]
    fn a_blend_pattern_averages_the_colour_at_all_points() {
        let p = Blend::new(Colour::red().into(), Colour::green().into());

        assert_approx_eq!(
            p.pattern_at(&Point::origin()),
            Colour::new(0.5, 0.5, 0.0)
        );

        assert_approx_eq!(
            p.pattern_at(&Point::new(1.5, 2.9, 0.3)),
            Colour::new(0.5, 0.5, 0.0)
        );
        assert_approx_eq!(
            p.pattern_at(&Point::new(-4.0, 0.21, -1.1)),
            Colour::new(0.5, 0.5, 0.0)
        );
    }

    add_kind_tests!(Blend);
}
