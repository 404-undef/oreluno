#![allow(dead_code)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/*
    TokenId
    ↓
    индекс vocabulary
    ↓
    char
*/

/// Ошибки токенизации
#[derive(Debug)]
pub enum TokenizerError {
    TokenNotFound(usize),
    CharNotFound(char),
    TokenIdPosOverflow,
    DuplicateToken(char),
    InternalError(String),
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenIdPosOverflow => write!(formatter, "Number too large to fit in `TokenId`"),
            Self::TokenNotFound(_) => todo!(),
            Self::CharNotFound(_) => todo!(),
            Self::DuplicateToken(arg) => write!(formatter, "Duplicate character `{arg}` after dedup"),
            Self::InternalError(_) => todo!(),
        }
    }
}

// Токен (минимальная единица, которой оперирует модель)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(u32);

impl TokenId {
    pub fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Список всех возможных токенов (Vocabulary)
#[derive(Debug, PartialEq, Eq)]
pub struct CharTokenizer {
    char_to_id: HashMap<char, TokenId>,
    id_to_char: Vec<char>,
}

impl CharTokenizer {
    /// Возвращает структуру (Vocabulary) содержащую все возможные токены из переданой строки `corpus`
    pub fn from_corpus(corpus: &str) -> Result<Self, TokenizerError> {
        /*
            собрали
            ↓
            отсортировали
            ↓
            dedup
            ↓
            назначили 0..N
        */

        let mut chars: Vec<char> = corpus.chars().collect();
        chars.sort_unstable();
        chars.dedup();

        let mut char_to_id = HashMap::with_capacity(chars.len());
        for (idx, ch) in chars.iter().enumerate() {
            let token_id = TokenId::from_index(idx).ok_or(TokenizerError::TokenIdPosOverflow)?;

            char_to_id
                .insert(*ch, token_id)
                .ok_or(TokenizerError::DuplicateToken(*ch))?;
        }

        Ok(Self {
            char_to_id,
            id_to_char: chars,
        })
    }

    /// text -> tokens
    pub fn encode(&self, text: &str) -> Result<Vec<TokenId>, TokenizerError> {
        let mut output = Vec::with_capacity(text.len());

        for ch in text.chars() {
            let token = self
                .char_to_id
                .get(&ch)
                .ok_or(TokenizerError::CharNotFound(ch))?;

            output.push(*token);
        }

        Ok(output)
    }

    /// tokens -> text
    pub fn decode(&self, tokens: &[TokenId]) -> Result<String, TokenizerError> {
        let mut output = String::new();

        for &id in tokens {
            let ch = self
                .id_to_char
                .get(id.index())
                .ok_or(TokenizerError::TokenNotFound(id.index()))?;

            output.push(*ch);
        }

        Ok(output)
    }

    pub fn vocab_size(&self) -> usize {
        self.char_to_id.len()
    }
}

// pub fn encode(string: &str) -> Vec<u32> {
//     string.chars().map(|c| c as u32).collect()
// }

// pub fn decode(tokens: &[u32]) -> String {
//     tokens
//         .iter()
//         .map(|&t| char::from_u32(t as u32).unwrap())
//         .collect()
// }
