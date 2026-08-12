use std::collections::{BTreeSet, HashMap};
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

/// Ошибки токенизации
#[derive(Debug)]
pub enum TokenizerError {
    UnknownTokenId(TokenId),
    UnknownChar(char),
    TokenIdPosOverflow,
    TokenIdToUsizeOverflow,
}

impl Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenIdPosOverflow => write!(formatter, "Number too large to fit in `TokenId`"),
            Self::TokenIdToUsizeOverflow => {
                write!(formatter, "TokenId value too large to fit in `usize`")
            }
            Self::UnknownTokenId(arg) => write!(formatter, "Unknown token id `{arg}`"),
            Self::UnknownChar(arg) => write!(formatter, "Unknown char `{arg}`"),
        }
    }
}

/// Компактный идентификатор токена в vocabulary
///
/// Для CharTokenizer токеном является `char`,
/// а TokenId хранит его позицию в vocabulary
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(u32);

impl TokenId {
    pub fn from_index(index: usize) -> Option<Self> {
        u32::try_from(index).ok().map(Self)
    }

    pub fn index(self) -> Option<usize> {
        usize::try_from(self.0).ok()
    }
}

impl fmt::Display for TokenId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/*
    TokenId
    ↓
    индекс vocabulary
    ↓
    char
*/

/// Character-level tokenizer с vocabulary,
/// построенным по символам обучающего корпуса
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
                .ok_or(TokenizerError::UnknownChar(ch))?;

            output.push(*token);
        }

        Ok(output)
    }

    /// tokens -> text
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

    pub fn vocab_size(&self) -> usize {
        self.id_to_char.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Вспомогательная функция для проверки round-trip
    fn assert_roundtrip(text: &str) {
        let tokenizer =
            CharTokenizer::from_corpus(text).expect("Failed to create tokenizer from corpus");

        let tokens = tokenizer.encode(text).expect("Failed to encode text");
        let decoded = tokenizer.decode(&tokens).expect("Failed to decode tokens");

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

        // Должен вернуть: CharNotFound('d')
        assert!(
            matches!(result, Err(TokenizerError::UnknownChar('d'))),
            "Expected Err(TokenizerError::UnknownChar('d')), but got {:?}",
            result
        );
    }

    #[test]
    fn deterministic_token_ids_by_alphabet() {
        // Корпус передан в обратном порядке: 'c', 'b', 'a'
        let corpus = "cba";
        let tokenizer = CharTokenizer::from_corpus(corpus).unwrap();

        // Кодируем строку в алфавитном порядке 'a', 'b', 'c'
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

        assert!(matches!(
            result,
            Err(TokenizerError::UnknownTokenId(TokenId(100)))
        ));
    }
}
