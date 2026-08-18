use crate::BigramStats;
use crate::RandomSource;
use crate::TokenId;
use crate::sample_index;
use std::{error::Error, fmt};

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
    stats: crate::BigramStats,
}

impl BigramModel {
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

/// Ошибки BigramStats
#[derive(Debug)]
pub enum BigramModelError {
    // probabilities(current)  -> ?
    // sample_index(...)       -> ?
    // TokenId::from_index()   -> ?
}

impl Error for BigramModelError {}

impl fmt::Display for BigramModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //...
}
