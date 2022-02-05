use rand::Rng;

const PERMUTATION_TABLE_SIZE: usize = 256;

/// An instance of improved perlin noise.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PerlinNoise {
    pub permutations: [u8; PERMUTATION_TABLE_SIZE * 2],
}

impl PerlinNoise {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut permutations = [0u8; PERMUTATION_TABLE_SIZE * 2];

        (0..PERMUTATION_TABLE_SIZE).for_each(|i| {
            permutations[i] = i as u8;
        });

        for i in (1..PERMUTATION_TABLE_SIZE).rev() {
            let j = rng.gen_range(0..i);

            permutations.swap(i, j);
        }

        (0..PERMUTATION_TABLE_SIZE).for_each(|i| {
            permutations[i + PERMUTATION_TABLE_SIZE] = permutations[i];
        });

        Self { permutations }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    use super::*;

    #[test]
    fn create_perlin_noise() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(1651);

        let p = PerlinNoise::new(&mut rng);

        for i in 0..PERMUTATION_TABLE_SIZE {
            assert_eq!(
                p.permutations[i],
                p.permutations[i + PERMUTATION_TABLE_SIZE]
            );
        }
    }
}
