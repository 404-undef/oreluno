#![allow(unused)]

use crate::TokenId;
use std::error::Error;
use std::fmt;

/// Статистика переходов между соседними токенами
///
/// Для vocabulary размера `V` хранит матрицу `V × V`,
/// где ячейка `(current, next)` содержит число наблюдений
/// перехода `current -> next`
///
/// Матрица хранится в одном `Vec<u64>` в row-major порядке
#[derive(Debug)]
pub struct BigramStats {
    counts: Vec<u64>,
    vocab_size: usize,
}

impl BigramStats {
    /// Создаёт пустую таблицу переходов для vocabulary размера `vocab_size`
    ///
    /// Все счётчики изначально равны нулю
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если:
    /// - `vocab_size == 0`
    /// - размер матрицы `vocab_size × vocab_size` не помещается в `usize`
    pub fn new(vocab_size: usize) -> Result<Self, BigramStatsError> {
        if vocab_size == 0 {
            return Err(BigramStatsError::EmptyVocabulary);
        }

        Ok(Self {
            counts: vec![
                0;
                vocab_size
                    .checked_mul(vocab_size)
                    .ok_or(BigramStatsError::MatrixSizeOverflow)?
            ],
            vocab_size,
        })
    }

    /// Добавляет статистику соседних токенов из `tokens`
    ///
    /// Для `[A, B, C]` учитываются переходы `A -> B` и `B -> C`
    /// Ранее накопленные счётчики не сбрасываются
    ///
    /// Пустой slice и slice из одного токена не изменяют статистику
    ///
    /// # Errors
    ///
    /// Возвращает ошибку при невалидном `TokenId` или переполнении счётчика
    pub fn observe(&mut self, tokens: &[TokenId]) -> Result<(), BigramStatsError> {
        todo!()
    }

    /// Возвращает число наблюдений перехода `current -> next`
    pub fn count(&self, current: TokenId, next: TokenId) -> Result<u64, BigramStatsError> {
        Ok(self.counts[self.index(current, next)?])
    }

    /// Возвращает распределение вероятностей следующего токена
    /// после `current`
    ///
    /// Элемент с индексом `j` равен эмпирической вероятности
    /// перехода `current -> TokenId(j)`
    ///
    /// Сумма элементов результата приблизительно равна `1.0`
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если:
    /// - `current` не принадлежит vocabulary
    /// - для `current` ещё не наблюдалось ни одного перехода
    pub fn probabilities(&self, current: TokenId) -> Result<Vec<f64>, BigramStatsError> {
        todo!()
    }

    /// Возвращает индекс перехода `(current, next)` в плоской row-major матрице
    fn index(&self, current: TokenId, next: TokenId) -> Result<usize, BigramStatsError> {
        let current_idx = self.validate_token(current)?;
        let next_idx = self.validate_token(next)?;

        // Row-major: сначала пропускаем `current` полных строк,
        // затем смещаемся до столбца `next`
        let index = current_idx
            .checked_mul(self.vocab_size)
            .and_then(|row| row.checked_add(next_idx))
            .ok_or(BigramStatsError::MatrixIndexOverflow)?;

        Ok(index)
    }

    /// Проверяет, что `token` принадлежит vocabulary,
    /// и возвращает его индекс как `usize`.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если:
    /// - значение `TokenId` невозможно представить как `usize`;
    /// - индекс токена находится за пределами vocabulary.
    fn validate_token(&self, token: TokenId) -> Result<usize, BigramStatsError> {
        let token_idx = token
            .index()
            .ok_or(BigramStatsError::TokenIndexConversionOverflow)?;

        if token_idx >= self.vocab_size {
            return Err(BigramStatsError::TokenOutOfVocabulary {
                token,
                vocab_size: self.vocab_size,
            });
        }

        Ok(token_idx)
    }
}

/// Ошибки, возникающие при создании и использовании [`BigramStats`]
#[derive(Debug, PartialEq, Eq)]
pub enum BigramStatsError {
    /// Токен не принадлежит словарю этой статистики
    TokenOutOfVocabulary {
        token: TokenId,
        vocab_size: usize,
    },
    /// Счётчик перехода достиг максимального значения `u64`
    CountOverflow {
        current: TokenId,
        next: TokenId,
    },
    /// Для токена ещё не наблюдалось исходящих переходов
    NoOutgoingTransitions(TokenId),
    /// Словарь не может быть пустым
    EmptyVocabulary,
    /// Матрица `V * V` не помещается в адресное пространство
    MatrixSizeOverflow,
    // Индекс ячейки матрицы не помещается в `usize`
    MatrixIndexOverflow,
    /// Значение `TokenId` невозможно представить как `usize`
    TokenIndexConversionOverflow,
}

impl Error for BigramStatsError {}

impl fmt::Display for BigramStatsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenOutOfVocabulary { token, vocab_size } => {
                write!(
                    formatter,
                    "Token id `{token}` is outside vocabulary of size `{vocab_size}`"
                )
            }

            Self::CountOverflow { current, next } => {
                write!(
                    formatter,
                    "Transition count overflow for `{current} -> {next}`"
                )
            }

            Self::NoOutgoingTransitions(token) => {
                write!(
                    formatter,
                    "No outgoing transitions observed for token `{token}`"
                )
            }

            Self::EmptyVocabulary => {
                write!(formatter, "Vocabulary must contain at least one token")
            }

            Self::MatrixSizeOverflow => {
                write!(
                    formatter,
                    "Bigram matrix size exceeds the maximum addressable `usize` range"
                )
            }

            Self::MatrixIndexOverflow => {
                write!(
                    formatter,
                    "Bigram matrix index exceeds the maximum `usize` value"
                )
            }

            Self::TokenIndexConversionOverflow => {
                write!(
                    formatter,
                    "Token id cannot be represented as `usize` on this platform"
                )
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_zeroed_counts() {
        let bigram_stats = BigramStats::new(3).unwrap();

        assert_eq!(bigram_stats.counts.len(), 3_usize.checked_mul(3).unwrap());

        assert!(bigram_stats.counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn new_rejects_empty_vocabulary() {
        let bigram_stats = BigramStats::new(0);

        assert!(
            matches!(bigram_stats, Err(BigramStatsError::EmptyVocabulary)),
            "Expected Err(BigramStatsError::EmptyVocabulary), but got {:?}",
            bigram_stats
        );
    }

    // Проверка правильной раскладки 2 * 2
    #[test]
    fn index_uses_row_major_layout() {
        let bigram_stats = BigramStats::new(2).unwrap();
        let t0 = TokenId::from_index(0_usize).unwrap();
        let t1 = TokenId::from_index(1_usize).unwrap();

        assert_eq!(bigram_stats.index(t0, t0).unwrap(), 0);
        assert_eq!(bigram_stats.index(t0, t1).unwrap(), 1);
        assert_eq!(bigram_stats.index(t1, t0).unwrap(), 2);
        assert_eq!(bigram_stats.index(t1, t1).unwrap(), 3);
    }

    #[test]
    fn index_reject_invalid_current() {
        let v = 2;
        let bigram_stats = BigramStats::new(v).unwrap();
        let current = TokenId::from_index(2_usize).unwrap();
        let next = TokenId::from_index(0_usize).unwrap();

        // Должен вернуть: TokenOutOfVocabulary
        assert!(
            matches!(
                bigram_stats.index(current, next),
                Err(BigramStatsError::TokenOutOfVocabulary {
                    token: current,
                    vocab_size: 2_usize,
                })
            ),
            "Expected Err(BigramStatsError::TokenOutOfVocabulary), but got {:?}",
            bigram_stats.index(current, next)
        );
    }

    #[test]
    fn index_reject_invalid_next() {
        let bigram_stats = BigramStats::new(2).unwrap();
        let current = TokenId::from_index(0_usize).unwrap();
        let next = TokenId::from_index(2_usize).unwrap();

        // Должен вернуть: TokenOutOfVocabulary
        assert!(
            matches!(
                bigram_stats.index(current, next),
                Err(BigramStatsError::TokenOutOfVocabulary {
                    token: next,
                    vocab_size: 2_usize,
                })
            ),
            "Expected Err(BigramStatsError::TokenOutOfVocabulary), but got {:?}",
            bigram_stats.index(current, next)
        );
    }
}
