//! deterministic PRNG

const GOLDEN_GAMMA: u64 = 0x9E3779B97F4A7C15;
const MIX_MULTIPLIER_1: u64 = 0xBF58476D1CE4E5B9;
const MIX_MULTIPLIER_2: u64 = 0x94D049BB133111EB;

#[derive(Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Функция перемешивания Stafford variant 13
    ///
    /// Значения сдвигов (30, 27, 31) и множители образуют
    /// специально подобранную 64-битную функцию перемешивания
    ///
    /// Они не выводятся независимо друг от друга и должны
    /// рассматриваться как единая конструкция
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);

        let mut random = self.state;
        random = (random ^ (random >> 30)).wrapping_mul(MIX_MULTIPLIER_1);
        random = (random ^ (random >> 27)).wrapping_mul(MIX_MULTIPLIER_2);
        random ^= random >> 31;

        random
    }

    pub fn next_f64(&mut self) -> f64 {
        let random = (self.next_u64() >> 11) as f64;

        random / (1_u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut rng_a = Rng::new(12345);
        let mut rng_b = Rng::new(12345);

        assert_eq!(rng_a.next_u64(), rng_b.next_u64());
        assert_eq!(rng_a.next_u64(), rng_b.next_u64());
        assert_eq!(rng_a.next_u64(), rng_b.next_u64());
    }

    #[test]
    fn next_f64_is_in_unit_interval() {
        let mut rng = Rng::new(12345);

        for _ in 0..1000 {
            let random = rng.next_f64();

            assert!(random >= 0.0);
            assert!(random < 1.0);
        }
    }

    #[test]
    fn same_seed_produces_same_f64_sequence() {
        let mut rng_a = Rng::new(12345);
        let mut rng_b = Rng::new(12345);

        assert_eq!(rng_a.next_f64(), rng_b.next_f64());
        assert_eq!(rng_a.next_f64(), rng_b.next_f64());
        assert_eq!(rng_a.next_f64(), rng_b.next_f64());
    }
}
