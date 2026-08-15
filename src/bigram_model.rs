#![allow(unused)]

use std::{error::Error, fmt};

use crate::TokenId;

/*
    TokenId
        ↓
    transition counts
        ↓
    probabilities
*/

// weights
// forward()
// loss()
// train_step()

struct BigramModel {
    counts: Vec<u64>,
    weights: Vec<f32>,
    vocab_size: usize,
}

impl BigramModel {
    pub fn new(vocab_size: usize) -> Result<Self, BigramModelError> {
        todo!()
    }

    fn train(&mut self, tokens: &[TokenId]) -> Result<(), BigramModelError> {
        todo!()
    }

    fn count(&self, current: TokenId, next: TokenId) -> Result<u64, BigramModelError> {
        todo!()
    }

    fn probabilities(&self, current: TokenId) -> Result<Vec<f64>, BigramModelError> {
        todo!()
    }
}

/// Ошибки BigramStats
#[derive(Debug)]
pub enum BigramModelError {
    
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