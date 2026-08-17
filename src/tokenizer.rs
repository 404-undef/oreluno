use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/*
    человеческий мир
        │
        │
    "Привет"
        │
        ▼
    TOKENIZER
        │
        ▼
    [4,8,7,5,6,9]
        │
        │
    мир модели
*/

/// Компактный числовой идентификатор токена
///
/// Значение соответствует позиции токена в vocabulary
/// Для [`CharTokenizer`] каждый `TokenId` соответствует одному Unicode `char`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(u32);

impl TokenId {
    /// Создаёт `TokenId` из индекса vocabulary
    ///
    /// Возвращает `None`, если `index` не помещается во внутренний `u32`
    pub fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    /// Возвращает числовое значение токена как индекс `usize`.
    ///
    /// Возвращает `None`, если значение невозможно представить как `usize`
    /// на текущей платформе
    pub fn index(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

impl fmt::Display for TokenId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Character-level tokenizer с детерминированным vocabulary
///
/// Каждый уникальный Unicode `char` корпуса получает отдельный [`TokenId`].
/// Идентификаторы назначаются в порядке сортировки символов
#[derive(Debug, PartialEq, Eq)]
pub struct CharTokenizer {
    char_to_id: HashMap<char, TokenId>,
    id_to_char: Vec<char>,
}

impl CharTokenizer {
    /// Строит tokenizer по уникальным символам `corpus`
    ///
    /// Символы сортируются перед назначением идентификаторов, поэтому
    /// одинаковый набор символов всегда создаёт одинаковое vocabulary
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если количество уникальных символов
    /// превышает диапазон, представимый [`TokenId`]
    pub fn from_corpus(corpus: &str) -> Result<Self, TokenizerError> {
        let unique_chars: BTreeSet<char> = corpus.chars().collect();
        let mut char_to_id = HashMap::with_capacity(unique_chars.len());
        let mut id_to_char = Vec::with_capacity(unique_chars.len());

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

    /// Кодирует `text` в последовательность идентификаторов токенов
    ///
    /// Порядок токенов совпадает с порядком символов исходного текста
    ///
    /// # Errors
    ///
    /// Возвращает [`TokenizerError::UnknownChar`], если `text` содержит
    /// символ, отсутствующий в vocabulary
    pub fn encode(&self, text: &str) -> Result<Vec<TokenId>, TokenizerError> {
        let mut output = Vec::with_capacity(text.len());

        for ch in text.chars() {
            let token = self
                .char_to_id
                .get(&ch)
                .ok_or(TokenizerError::UnknownChar(ch))?;

            output.push(*token);
        }

        Ok(output)
    }

    /// Декодирует последовательность токенов обратно в строку
    ///
    /// # Errors
    ///
    /// Возвращает:
    /// - [`TokenizerError::TokenIdToUsizeOverflow`], если значение токена невозможно представить как `usize`
    /// - [`TokenizerError::UnknownTokenId`], если идентификатор отсутствует в vocabulary
    pub fn decode(&self, tokens: &[TokenId]) -> Result<String, TokenizerError> {
        let mut output = String::with_capacity(tokens.len());

        for &id in tokens {
            let token_idx = id.index().ok_or(TokenizerError::TokenIdToUsizeOverflow)?;

            let ch = self
                .id_to_char
                .get(token_idx)
                .ok_or(TokenizerError::UnknownTokenId(id))?;

            output.push(*ch);
        }

        Ok(output)
    }

    /// Возвращает количество токенов в vocabulary
    pub fn vocab_size(&self) -> usize {
        self.id_to_char.len()
    }
}

/// Ошибки, возникающие при создании и использовании [`CharTokenizer`]
#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    /// Идентификатор токена отсутствует в vocabulary
    UnknownTokenId(TokenId),
    /// Символ отсутствует в vocabulary
    UnknownChar(char),
    /// Позиция токена не помещается во внутреннее представление [`TokenId`]
    TokenIdPosOverflow,
    /// Значение [`TokenId`] невозможно представить как `usize`
    TokenIdToUsizeOverflow,
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTokenId(token) => {
                write!(formatter, "Unknown token id `{token}`")
            }

            Self::UnknownChar(ch) => {
                write!(formatter, "Character `{ch}` is not present in vocabulary")
            }

            Self::TokenIdPosOverflow => {
                write!(
                    formatter,
                    "Vocabulary contains more tokens than `TokenId` can represent"
                )
            }

            Self::TokenIdToUsizeOverflow => {
                write!(
                    formatter,
                    "Token id cannot be represented as `usize` on this platform"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_roundtrip(text: &str) {
        let tokenizer = CharTokenizer::from_corpus(text).unwrap();
        let tokens = tokenizer.encode(text).unwrap();
        let decoded = tokenizer.decode(&tokens).unwrap();

        assert_eq!(decoded, text);
    }

    #[test]
    fn ascii_roundtrip() {
        assert_roundtrip("hello world");
    }

    #[test]
    fn cyrillic_roundtrip() {
        assert_roundtrip("Привет, мир!");
    }

    #[test]
    fn mixed_utf8_roundtrip() {
        assert_roundtrip("Hello, мир! 你好 🚀");
    }

    #[test]
    fn encode_unknown_char_returns_error() {
        let tokenizer = CharTokenizer::from_corpus("abc").unwrap();
        let result = tokenizer.encode("abcd");

        assert_eq!(result, Err(TokenizerError::UnknownChar('d')));
    }

    #[test]
    fn deterministic_token_ids_by_alphabet() {
        let tokenizer = CharTokenizer::from_corpus("cba").unwrap();
        let tokens = tokenizer.encode("abc").unwrap();
        let indices: Vec<usize> = tokens
            .into_iter()
            .map(|token| token.index().unwrap())
            .collect();

        assert_eq!(indices, vec![0, 1, 2]);
    }

    #[test]
    fn decode_unknown_token_id_returns_error() {
        let tokenizer = CharTokenizer::from_corpus("abc").unwrap();
        let tokens = [TokenId(100)];
        let result = tokenizer.decode(&tokens);

        assert_eq!(result, Err(TokenizerError::UnknownTokenId(TokenId(100))));
    }
}
