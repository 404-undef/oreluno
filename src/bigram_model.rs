use crate::RandomSource;
use crate::TokenId;
use crate::{BigramStats, BigramStatsError};
use crate::{SamplingError, sample_index};
use std::error::Error;
use std::fmt;


pub struct BigramModel {
    stats: BigramStats,
}

impl BigramModel {
    pub fn new(stats: BigramStats) -> Self {
        Self { stats }
    }

    pub fn next_token(
        &self,
        current: TokenId,
        rng: &mut impl RandomSource,
    ) -> Result<TokenId, BigramModelError> {
        let index = sample_index(&self.stats.probabilities(current)?, rng.next_f64())?;

        TokenId::from_index(index).ok_or(BigramModelError::InvalidTokenIndex(index))
    }
}

/// Ошибки, возникающие при работе bigram-модели
///
/// Объединяет ошибки получения статистики переходов,
/// выбора токена из распределения вероятностей и
/// преобразования индекса в [`TokenId`]
#[derive(Debug)]
pub enum BigramModelError {
    Stats(BigramStatsError),
    Sampling(SamplingError),
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
}
