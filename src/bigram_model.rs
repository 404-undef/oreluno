//! Bigram-модель для выбора следующего токена.
//!
//! Модель связывает статистику переходов, источник
//! псевдослучайных значений и sampling:
//!
//! ```text
//! current TokenId
//!     ↓
//! BigramStats::probabilities(current)
//!     ↓
//! RandomSource::next_f64()
//!     ↓
//! sample_index(...)
//!     ↓
//! next TokenId
//! ```

use crate::RandomSource;
use crate::TokenId;
use crate::{BigramStats, BigramStatsError};
use crate::{SamplingError, sample_index};
use std::error::Error;
use std::fmt;

/// Bigram-модель, выбирающая следующий токен
/// на основе статистики переходов между токенами
///
/// Модель хранит [`BigramStats`] и использует внешний
/// [`RandomSource`] для случайного выбора следующего токена
pub struct BigramModel {
    stats: BigramStats,
}

impl BigramModel {
    /// Создаёт bigram-модель из готовой статистики переходов
    pub fn new(stats: BigramStats) -> Self {
        Self { stats }
    }

    /// Выбирает следующий токен для заданного текущего токена
    ///
    /// Получает распределение вероятностей из [`BigramStats`],
    /// получает случайное значение из [`RandomSource`] и выполняет sampling
    ///
    /// # Errors
    ///
    /// Возвращает [`BigramModelError`], если:
    /// - не удалось получить распределение вероятностей
    /// - sampling завершился ошибкой
    /// - выбранный индекс невозможно преобразовать в [`TokenId`]
    pub fn next_token(
        &self,
        current: TokenId,
        rng: &mut impl RandomSource,
    ) -> Result<TokenId, BigramModelError> {
        let index = sample_index(&self.stats.probabilities(current)?, rng.next_f64())?;

        TokenId::from_index(index).ok_or(BigramModelError::InvalidTokenIndex(index))
    }

    /// Генерирует последовательность токенов, начиная с заданного токена
    ///
    /// `current` используется как начальный контекст и не включается
    /// в возвращаемую последовательность
    ///
    /// Генерируется ровно `count` новых токенов. Каждый выбранный токен
    /// становится текущим для следующего шага
    ///
    /// При `count == 0` возвращается пустой вектор
    ///
    /// # Errors
    ///
    /// Возвращает [`BigramModelError`], если на любом шаге
    /// не удалось выбрать следующий токен.
    pub fn generate(
        &self,
        current: TokenId,
        count: usize,
        rng: &mut impl RandomSource,
    ) -> Result<Vec<TokenId>, BigramModelError> {
        let mut generated = Vec::with_capacity(count);
        let mut current = current;

        for _ in 0..count {
            current = self.next_token(current, rng)?;
            generated.push(current);
        }

        Ok(generated)
    }
}

/// Ошибки, возникающие при работе bigram-модели
///
/// Объединяет ошибки получения статистики переходов,
/// выбора токена из распределения вероятностей и
/// преобразования индекса в [`TokenId`]
#[derive(Debug)]
pub enum BigramModelError {
    /// Ошибка при работе со статистикой bigram-переходов
    Stats(BigramStatsError),

    /// Ошибка при выборе токена из распределения вероятностей
    Sampling(SamplingError),

    /// Выбранный индекс невозможно представить как [`TokenId`]
    InvalidTokenIndex(usize),
}

impl From<BigramStatsError> for BigramModelError {
    fn from(error: BigramStatsError) -> Self {
        Self::Stats(error)
    }
}

impl From<SamplingError> for BigramModelError {
    fn from(error: SamplingError) -> Self {
        Self::Sampling(error)
    }
}

impl Error for BigramModelError {}

impl fmt::Display for BigramModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stats(error) => {
                write!(formatter, "bigram statistics error: {error}")
            }
            Self::Sampling(error) => {
                write!(formatter, "sampling error: {error}")
            }
            Self::InvalidTokenIndex(index) => {
                write!(formatter, "invalid token index: {index}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRandom {
        value: f64,
    }

    impl FixedRandom {
        fn new(value: f64) -> Self {
            Self { value }
        }
    }

    impl RandomSource for FixedRandom {
        fn next_f64(&mut self) -> f64 {
            self.value
        }
    }

    fn token(index: usize) -> TokenId {
        TokenId::from_index(index).unwrap()
    }

    fn model_with_split_distribution() -> BigramModel {
        let mut stats = BigramStats::new(3).unwrap();

        // Переходы:
        // 0 -> 1
        // 1 -> 0
        // 0 -> 2
        //
        // Для token 0:
        // P(0) = 0.0
        // P(1) = 0.5
        // P(2) = 0.5
        stats
            .observe(&[token(0), token(1), token(0), token(2)])
            .unwrap();

        BigramModel::new(stats)
    }

    #[test]
    fn next_token_selects_first_probability_interval() {
        let model = model_with_split_distribution();
        let mut rng = FixedRandom::new(0.0);

        let next = model.next_token(token(0), &mut rng).unwrap();

        assert_eq!(next, token(1));
    }

    #[test]
    fn next_token_selects_second_probability_interval() {
        let model = model_with_split_distribution();
        let mut rng = FixedRandom::new(0.5);

        let next = model.next_token(token(0), &mut rng).unwrap();

        assert_eq!(next, token(2));
    }

    #[test]
    fn next_token_propagates_stats_error() {
        let mut stats = BigramStats::new(2).unwrap();
        stats.observe(&[token(0), token(1)]).unwrap();

        let model = BigramModel::new(stats);
        let mut rng = FixedRandom::new(0.5);

        let result = model.next_token(token(1), &mut rng);

        assert!(matches!(
            result,
            Err(BigramModelError::Stats(
                BigramStatsError::NoOutgoingTransitions(_)
            ))
        ));
    }

    #[test]
    fn next_token_propagates_sampling_error() {
        let model = model_with_split_distribution();
        let mut rng = FixedRandom::new(1.0);

        let result = model.next_token(token(0), &mut rng);

        assert!(matches!(
            result,
            Err(BigramModelError::Sampling(
                SamplingError::InvalidRandomValue(_)
            ))
        ));
    }

    #[test]
    fn generate_returns_requested_number_of_tokens() {
        let model = model_with_split_distribution();
        let mut rng = FixedRandom::new(0.0);

        let generated = model.generate(token(0), 3, &mut rng).unwrap();

        assert_eq!(generated.len(), 3);
    }

    #[test]
    fn generate_returns_empty_sequence_for_zero_count() {
        let model = model_with_split_distribution();
        let mut rng = FixedRandom::new(0.0);

        let generated = model.generate(token(0), 0, &mut rng).unwrap();

        assert!(generated.is_empty());
    }

    #[test]
    fn generate_uses_previous_token_as_next_context() {
        let mut stats = BigramStats::new(3).unwrap();

        // Детерминированная цепочка:
        // 0 -> 1 -> 2
        stats.observe(&[token(0), token(1), token(2)]).unwrap();

        let model = BigramModel::new(stats);
        let mut rng = FixedRandom::new(0.0);

        let generated = model.generate(token(0), 2, &mut rng).unwrap();

        assert_eq!(generated, vec![token(1), token(2)]);
    }
}
