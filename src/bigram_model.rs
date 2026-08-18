use crate::RandomSource;
use crate::TokenId;
use crate::{BigramStats, BigramStatsError};
use crate::{SamplingError, sample_index};
use std::error::Error;
use std::fmt;

/*
    current TokenId
        ↓
    BigramStats::probabilities(current)
        ↓
    распределение вероятностей
        ↓
    RandomSource::next_f64()
        ↓
    sample_index(...)
        ↓
    next TokenId
        ↓
    он становится новым current
        ↓
    повтор
*/

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
        // 1. probabilities(current)
        // 2. rng.next_f64()
        // 3. sample_index(...)
        // 4. usize → TokenId
        todo!()
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
mod test {
    use super::*;

    //...
}
