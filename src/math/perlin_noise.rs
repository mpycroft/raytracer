use std::marker::PhantomData;

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::ToPrimitive;
use rand::{prelude::SliceRandom, Rng};

use super::{lerp, Point};
use crate::util::{
    approx::{FLOAT_EPSILON, FLOAT_ULPS},
    float::Float,
};

const PERMUTATION_TABLE_SIZE: usize = 256;
const PERMUTATION_TABLE_MASK: usize = PERMUTATION_TABLE_SIZE - 1;

/// An instance of improved perlin noise.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PerlinNoise<T: Float> {
    permutations: [u8; PERMUTATION_TABLE_SIZE * 2],
    _phantom: PhantomData<T>,
}

impl<T: Float> PerlinNoise<T> {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut permutation_values: Vec<u8> =
            (0..PERMUTATION_TABLE_SIZE).map(|val| val as u8).collect();

        permutation_values.shuffle(rng);

        let mut permutations = [0u8; PERMUTATION_TABLE_SIZE * 2];

        for (a, b) in
            permutations.iter_mut().zip(permutation_values.repeat(2).iter())
        {
            *a = *b;
        }

        Self { permutations, _phantom: PhantomData }
    }

    /// Returns Perlin noise for the given 3d point but shifted to be in the
    /// range 0..1.
    pub fn get_noise(&self, point: &Point<T>) -> T {
        (self.get_noise_signed(point) + T::one()) / T::two()
    }

    /// Returns Perlin noise for the given 3d point but shifted to be in the
    /// range 0..1.
    pub fn get_noise_signed(&self, point: &Point<T>) -> T {
        // This is sqrt(3.0) / 2.0 but sqrt() can't be used in const contexts
        const FACTOR: f64 = 0.866_025_403_784_438_6;

        self.raw_noise(point) / T::convert(FACTOR)
    }

    /// Return the raw Perlin noise for a given 3d point, returns values in the
    /// range -sqrt(3)/2 to sqrt(3)/2.
    fn raw_noise(&self, point: &Point<T>) -> T {
        let x0 = (ToPrimitive::to_isize(&point.x.floor())
            .expect("Converting to isize failed") as usize)
            & PERMUTATION_TABLE_MASK;
        let y0 = (ToPrimitive::to_isize(&point.y.floor())
            .expect("Converting to isize failed") as usize)
            & PERMUTATION_TABLE_MASK;
        let z0 = (ToPrimitive::to_isize(&point.z.floor())
            .expect("Converting to isize failed") as usize)
            & PERMUTATION_TABLE_MASK;

        let x1 = (x0 + 1) & PERMUTATION_TABLE_MASK;
        let y1 = (y0 + 1) & PERMUTATION_TABLE_MASK;
        let z1 = (z0 + 1) & PERMUTATION_TABLE_MASK;

        let x = point.x - point.x.floor();
        let y = point.y - point.y.floor();
        let z = point.z - point.z.floor();

        let fx = Self::fade(x);
        let fy = Self::fade(y);
        let fz = Self::fade(z);

        lerp(
            lerp(
                lerp(
                    Self::gradient(self.hash(x0, y0, z0), x, y, z),
                    Self::gradient(self.hash(x1, y0, z0), x - T::one(), y, z),
                    fx,
                ),
                lerp(
                    Self::gradient(self.hash(x0, y1, z0), x, y - T::one(), z),
                    Self::gradient(
                        self.hash(x1, y1, z0),
                        x - T::one(),
                        y - T::one(),
                        z,
                    ),
                    fx,
                ),
                fy,
            ),
            lerp(
                lerp(
                    Self::gradient(self.hash(x0, y0, z1), x, y, z - T::one()),
                    Self::gradient(
                        self.hash(x1, y0, z1),
                        x - T::one(),
                        y,
                        z - T::one(),
                    ),
                    fx,
                ),
                lerp(
                    Self::gradient(
                        self.hash(x0, y1, z1),
                        x,
                        y - T::one(),
                        z - T::one(),
                    ),
                    Self::gradient(
                        self.hash(x1, y1, z1),
                        x - T::one(),
                        y - T::one(),
                        z - T::one(),
                    ),
                    fx,
                ),
                fy,
            ),
            fz,
        )
    }

    fn hash(&self, x: usize, y: usize, z: usize) -> u8 {
        self.permutations
            [self.permutations[self.permutations[x] as usize + y] as usize + z]
    }

    fn fade(t: T) -> T {
        assert!(t >= T::zero() && t <= T::one());

        ((T::convert(6.0f64) * t - T::convert(15.0f64)) * t
            + T::convert(10.0f64))
            * t
            * t
            * t
    }

    fn gradient(hash: u8, x: T, y: T, z: T) -> T {
        match hash & 15 {
            0 | 12 => x + y,
            1 | 13 => -x + y,
            2 => x - y,
            3 => -x - y,
            4 => x + z,
            5 => -x + z,
            6 => x - z,
            7 => -x - z,
            8 => y + z,
            9 | 14 => -y + z,
            10 => y - z,
            11 | 15 => -y - z,
            _ => unreachable!(),
        }
    }
}

impl<T> AbsDiffEq for PerlinNoise<T>
where
    T: Float + AbsDiffEq,
    T::Epsilon: Float,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::Epsilon::convert(FLOAT_EPSILON)
    }

    fn abs_diff_eq(&self, other: &Self, _epsilon: Self::Epsilon) -> bool {
        self.permutations
            .iter()
            .zip(other.permutations.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T> RelativeEq for PerlinNoise<T>
where
    T: Float + RelativeEq,
    T::Epsilon: Float,
{
    fn default_max_relative() -> Self::Epsilon {
        T::Epsilon::convert(FLOAT_EPSILON)
    }

    fn relative_eq(
        &self,
        other: &Self,
        _epsilon: Self::Epsilon,
        _max_relative: Self::Epsilon,
    ) -> bool {
        self.permutations
            .iter()
            .zip(other.permutations.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T> UlpsEq for PerlinNoise<T>
where
    T: Float + UlpsEq,
    T::Epsilon: Float,
{
    fn default_max_ulps() -> u32 {
        FLOAT_ULPS
    }

    fn ulps_eq(
        &self,
        other: &Self,
        _epsilon: Self::Epsilon,
        _max_ulps: u32,
    ) -> bool {
        self.permutations
            .iter()
            .zip(other.permutations.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;

    // Better testing would be ideal but given the random nature of the
    // permutation table it would prove difficult. We could have an option to
    // use a fixed table (ala Perlin's original implementation) then test some
    // points with a different implementation. Though the benefits once it
    // "looks right" are debatable.

    #[test]
    fn create_perlin_noise() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(1651);

        let p = PerlinNoise::<f64>::new(&mut rng);

        for i in 0..PERMUTATION_TABLE_SIZE {
            assert_eq!(
                p.permutations[i],
                p.permutations[i + PERMUTATION_TABLE_SIZE]
            );
        }
    }

    #[test]
    fn fade_produces_expected_values() {
        assert_float_relative_eq!(PerlinNoise::fade(0.0), 0.0);
        assert_float_relative_eq!(PerlinNoise::fade(1.0), 1.0);

        assert_float_relative_eq!(PerlinNoise::fade(0.2), 0.057_92);
        assert_float_relative_eq!(PerlinNoise::fade(0.5), 0.5);
        assert_float_relative_eq!(PerlinNoise::fade(0.68), 0.809_474);
    }

    #[test]
    #[should_panic]
    fn fade_should_assert_on_negative_values() {
        let _ = PerlinNoise::fade(-0.5);
    }

    #[test]
    #[should_panic]
    fn fade_should_assert_on_values_greater_than_1() {
        let _ = PerlinNoise::fade(1.9);
    }

    #[test]
    fn gradient_produces_expected_values() {
        let g = |hash| PerlinNoise::gradient(hash, 1.0, 2.0, 3.0);

        assert_float_relative_eq!(g(0), 3.0);
        assert_float_relative_eq!(g(12), 3.0);
        assert_float_relative_eq!(g(16), 3.0);
        assert_float_relative_eq!(g(44), 3.0);

        assert_float_relative_eq!(g(1), 1.0);
        assert_float_relative_eq!(g(13), 1.0);
        assert_float_relative_eq!(g(17), 1.0);
        assert_float_relative_eq!(g(30), 1.0);

        assert_float_relative_eq!(g(2), -1.0);
        assert_float_relative_eq!(g(18), -1.0);

        assert_float_relative_eq!(g(3), -3.0);
        assert_float_relative_eq!(g(35), -3.0);

        assert_float_relative_eq!(g(4), 4.0);
        assert_float_relative_eq!(g(5), 2.0);
        assert_float_relative_eq!(g(6), -2.0);
        assert_float_relative_eq!(g(7), -4.0);
        assert_float_relative_eq!(g(8), 5.0);

        assert_float_relative_eq!(g(9), 1.0);
        assert_float_relative_eq!(g(14), 1.0);

        assert_float_relative_eq!(g(10), -1.0);

        assert_float_relative_eq!(g(11), -5.0);
        assert_float_relative_eq!(g(15), -5.0);
    }

    #[test]
    fn perlin_noises_are_approximately_equal() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(621);
        let p1 = PerlinNoise::<f64>::new(&mut rng);
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(621);
        let p2 = PerlinNoise::new(&mut rng);
        let p3 = PerlinNoise::new(&mut rng);

        assert_abs_diff_eq!(p1, p2);
        assert_abs_diff_ne!(p1, p3);

        assert_relative_eq!(p1, p2);
        assert_relative_ne!(p1, p3);

        assert_ulps_eq!(p1, p2);
        assert_ulps_ne!(p1, p3);
    }
}
