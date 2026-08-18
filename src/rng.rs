//! Детерминированный генератор псевдослучайных чисел
//!
//! Модуль содержит учебную реализацию генератора на основе SplitMix64
//!
//! Генератор полностью детерминирован: одинаковое начальное значение
//! `seed` всегда приводит к одинаковой последовательности результатов
//!
//! [`Rng`] предоставляет конкретную реализацию генератора, а
//! [`RandomSource`] задаёт минимальный контракт источника случайных
//! значений, который позволяет в дальнейшем подменять реализацию RNG

pub const GOLDEN_GAMMA: u64 = 0x9E3779B97F4A7C15;
pub const MIX_MULTIPLIER_1: u64 = 0xBF58476D1CE4E5B9;
pub const MIX_MULTIPLIER_2: u64 = 0x94D049BB133111EB;

/// Детерминированный генератор псевдослучайных чисел на основе SplitMix64
///
/// Генератор хранит 64-битное внутреннее состояние и при каждом вызове
/// [`Rng::next_u64`] переходит к следующему состоянию, после чего применяет
/// 64-битную функцию перемешивания Stafford variant 13
///
/// Одинаковый `seed` всегда создаёт одинаковую последовательность значений
#[derive(Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Создаёт генератор с заданным начальным состоянием
    ///
    /// # Arguments
    ///
    /// * `seed` - начальное 64-битное состояние генератора
    ///
    /// Значение `0` является допустимым seed и не требует специальной
    /// обработки
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Возвращает следующее псевдослучайное 64-битное значение.
    ///
    /// Сначала внутреннее состояние увеличивается на [`GOLDEN_GAMMA`]
    /// с арифметикой по модулю `2^64`. Затем полученное значение
    /// перемешивается функцией Stafford variant 13
    ///
    /// Значения сдвигов `30`, `27`, `31` и множители
    /// [`MIX_MULTIPLIER_1`] и [`MIX_MULTIPLIER_2`] образуют специально
    /// подобранную 64-битную функцию перемешивания и рассматриваются
    /// как единая конструкция
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GOLDEN_GAMMA);

        let mut random = self.state;
        random = (random ^ (random >> 30)).wrapping_mul(MIX_MULTIPLIER_1);
        random = (random ^ (random >> 27)).wrapping_mul(MIX_MULTIPLIER_2);
        random ^= random >> 31;

        random
    }

    /// Возвращает следующее псевдослучайное число в диапазоне `[0.0, 1.0)`
    ///
    /// Из результата [`Rng::next_u64`] используются старшие 53 бита,
    /// поскольку `f64` имеет 53 бита точности значащей части
    ///
    /// Значение сдвига `11` получается как:
    ///
    /// `64 - 53 = 11`
    ///
    /// После сдвига результат находится в диапазоне от `0` до
    /// `2^53 - 1` и делится на `2^53`, поэтому значение `1.0`
    /// никогда не возвращается
    pub fn next_f64(&mut self) -> f64 {
        let random = (self.next_u64() >> 11) as f64;

        random / (1_u64 << 53) as f64
    }
}

/// Источник псевдослучайных значений для алгоритмов библиотеки
///
/// Trait отделяет потребителя случайных чисел от конкретной реализации
/// генератора. Благодаря этому алгоритмы могут использовать [`Rng`] либо
/// другую пользовательскую или production-реализацию, не меняя свою
/// основную логику
///
/// Реализация должна возвращать значения в диапазоне `[0.0, 1.0)`
pub trait RandomSource {
    fn next_f64(&mut self) -> f64;
}

impl RandomSource for Rng {
    fn next_f64(&mut self) -> f64 {
        Rng::next_f64(self)
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
