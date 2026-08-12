#![allow(dead_code)]

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
    pub fn new(vocab_size: usize) -> Result<Self, BigramError>;

    pub fn train(&mut self, tokens: &[TokenId]) -> Result<(), BigramError>;

    pub fn count(
        &self,
        current: TokenId,
        next: TokenId,
    ) -> Result<u64, BigramError>;

    pub fn probabilities(
        &self,
        current: TokenId,
    ) -> Result<Vec<f64>, BigramError>;
}
