#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap};
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
    InternalError(String),
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenIdPosOverflow => write!(formatter, "Number too large to fit in `TokenId`"),
            Self::TokenNotFound(arg) => write!(formatter, "Token `{arg}` not found"),
            Self::CharNotFound(arg) => write!(formatter, "Char `{arg}` not found"),
            Self::InternalError(arg) => write!(formatter, "{arg}"),
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
            убрали дубликаты
            ↓
            назначили 0..N
        */

        // 1. Собираем уникальные символы сразу в отсортированном виде
        let unique_chars: BTreeSet<char> = corpus.chars().collect();

        // 2. Выделяем память ровно под нужное количество элементов
        let mut char_to_id = HashMap::with_capacity(unique_chars.len());
        let mut id_to_char = Vec::with_capacity(unique_chars.len());

        // 3. Заполняем обе структуры за один проход
        for (idx, &ch) in unique_chars.iter().enumerate() {
            let token_id = TokenId::from_index(idx).ok_or(TokenizerError::TokenIdPosOverflow)?;

            char_to_id.insert(ch, token_id);
            id_to_char.push(ch);
        }

        Ok(Self {
            char_to_id,
            id_to_char,
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
