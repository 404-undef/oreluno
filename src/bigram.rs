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

pub struct BigramModel {
    counts: Vec<u64>,
    weights: Vec<f32>,
    vocab_size: usize,
}

impl BigramModel {
    pub fn new(vocab_size: usize) -> Result<Self, BigramError> {
        todo!()
    }

    fn train(&mut self, tokens: &[TokenId]) -> Result<(), BigramError> {
        todo!()
    }

    fn count(&self, current: TokenId, next: TokenId) -> Result<u64, BigramError> {
        todo!()
    }

    fn probabilities(&self, current: TokenId) -> Result<Vec<f64>, BigramError> {
        todo!()
    }
}

/// Ошибки BigramModel
#[derive(Debug)]
pub enum BigramError {
    
}

impl Error for BigramError {}

impl fmt::Display for BigramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
