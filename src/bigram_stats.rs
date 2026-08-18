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
#[derive(Debug, PartialEq)]
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
    /// Возвращает ошибку при невалидном `TokenId` или
    /// [`BigramStatsError::CountOverflow`] при переполнении счётчика
    pub fn observe(&mut self, tokens: &[TokenId]) -> Result<(), BigramStatsError> {
        for window in tokens.windows(2) {
            let current = window[0];
            let next = window[1];
            let index = self.index(current, next)?;

            self.counts[index] = self.counts[index]
                .checked_add(1)
                .ok_or(BigramStatsError::CountOverflow { current, next })?;
        }

        Ok(())
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
        let current_idx = self.validate_token(current)?;
        let start = current_idx * self.vocab_size;
        let end = start + self.vocab_size;
        let row = &self.counts[start..end];

        let total = row.iter().try_fold(0_u64, |acc, &count| {
            acc.checked_add(count)
                .ok_or(BigramStatsError::OutgoingCountOverflow(current))
        })?;

        if total == 0 {
            return Err(BigramStatsError::NoOutgoingTransitions(current));
        }

        let probabilities = row
            .iter()
            .map(|&count| count as f64 / total as f64)
            .collect();

        Ok(probabilities)
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
    TokenOutOfVocabulary { token: TokenId, vocab_size: usize },

    /// Счётчик перехода достиг максимального значения `u64`
    CountOverflow { current: TokenId, next: TokenId },

    /// Для токена ещё не наблюдалось исходящих переходов
    NoOutgoingTransitions(TokenId),

    /// Словарь не может быть пустым
    EmptyVocabulary,

    /// Матрица `V * V` не помещается в адресное пространство
    MatrixSizeOverflow,

    /// Индекс ячейки матрицы не помещается в `usize`
    MatrixIndexOverflow,

    /// Значение `TokenId` невозможно представить как `usize`
    TokenIndexConversionOverflow,

    /// Сумма счётчиков всех исходящих переходов токена не помещается в `u64`
    OutgoingCountOverflow(TokenId),
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

            Self::OutgoingCountOverflow(token) => {
                write!(
                    formatter,
                    "Total outgoing transition count overflow for token `{token}`"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_zeroed_counts() {
        let stats = BigramStats::new(3).unwrap();

        assert_eq!(stats.counts.len(), 3_usize.checked_mul(3).unwrap());
        assert!(stats.counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn new_rejects_empty_vocabulary() {
        assert_eq!(BigramStats::new(0), Err(BigramStatsError::EmptyVocabulary));
    }

    #[test]
    fn index_uses_row_major_layout() {
        let t0 = TokenId::from_index(0).unwrap();
        let t1 = TokenId::from_index(1).unwrap();
        let stats = BigramStats::new(2).unwrap();

        assert_eq!(stats.index(t0, t0).unwrap(), 0);
        assert_eq!(stats.index(t0, t1).unwrap(), 1);
        assert_eq!(stats.index(t1, t0).unwrap(), 2);
        assert_eq!(stats.index(t1, t1).unwrap(), 3);
    }

    #[test]
    fn index_rejects_invalid_current() {
        let v = 2;
        let stats = BigramStats::new(v).unwrap();
        let current = TokenId::from_index(2).unwrap();
        let next = TokenId::from_index(0).unwrap();

        assert_eq!(
            stats.index(current, next),
            Err(BigramStatsError::TokenOutOfVocabulary {
                token: current,
                vocab_size: 2,
            })
        );
    }

    #[test]
    fn index_rejects_invalid_next() {
        let stats = BigramStats::new(2).unwrap();
        let current = TokenId::from_index(0).unwrap();
        let next = TokenId::from_index(2).unwrap();

        assert_eq!(
            stats.index(current, next),
            Err(BigramStatsError::TokenOutOfVocabulary {
                token: next,
                vocab_size: 2_usize,
            })
        );
    }

    #[test]
    fn observe_counts_bigrams() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let tokens = [a, b, b, a];
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&tokens).unwrap();

        assert_eq!(stats.count(a, a).unwrap(), 0);
        assert_eq!(stats.count(a, b).unwrap(), 1);
        assert_eq!(stats.count(b, a).unwrap(), 1);
        assert_eq!(stats.count(b, b).unwrap(), 1);
    }

    #[test]
    fn observe_accumulates_counts() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&[a, b]).unwrap();
        stats.observe(&[a, b]).unwrap();

        assert_eq!(stats.count(a, b).unwrap(), 2);
    }

    #[test]
    fn observe_empty_slice_does_nothing() {
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&[]).unwrap();

        assert!(stats.counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn observe_single_token_does_nothing() {
        let a = TokenId::from_index(0).unwrap();
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&[a]).unwrap();

        assert!(stats.counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn observe_rejects_out_of_vocabulary_token() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(2).unwrap();
        let mut stats = BigramStats::new(2).unwrap();

        assert_eq!(
            stats.observe(&[a, b]),
            Err(BigramStatsError::TokenOutOfVocabulary {
                token: b,
                vocab_size: stats.vocab_size
            })
        );
    }

    #[test]
    fn observe_rejects_count_overflow() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let mut stats = BigramStats::new(2).unwrap();
        let index = stats.index(a, b).unwrap();
        stats.counts[index] = u64::MAX;

        assert_eq!(
            stats.observe(&[a, b]),
            Err(BigramStatsError::CountOverflow {
                current: a,
                next: b,
            })
        );
    }

    #[test]
    fn probabilities_normalize_counts() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let tokens = [a, b, b, a];
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&tokens).unwrap();

        assert_eq!(stats.probabilities(a).unwrap(), &[0.0, 1.0]);
        assert_eq!(stats.probabilities(b).unwrap(), &[0.5, 0.5]);
    }

    #[test]
    fn probabilities_sum_to_one() {
        let epsilon = 1e-12;
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let c = TokenId::from_index(2).unwrap();
        let tokens = [a, a, b, a, c];
        let mut stats = BigramStats::new(3).unwrap();

        stats.observe(&tokens).unwrap();
        let sum: f64 = stats.probabilities(a).unwrap().iter().sum();

        assert!((sum - 1.0).abs() < epsilon);
    }

    #[test]
    fn probabilities_rejects_invalid_token() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let c = TokenId::from_index(2).unwrap();
        let tokens = [a, b, b, a];
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&tokens).unwrap();

        assert_eq!(
            stats.probabilities(c),
            Err(BigramStatsError::TokenOutOfVocabulary {
                token: c,
                vocab_size: 2
            })
        );
    }

    #[test]
    fn probabilities_rejects_token_without_outgoing_transitions() {
        let a = TokenId::from_index(0).unwrap();
        let b = TokenId::from_index(1).unwrap();
        let tokens = [a, b];
        let mut stats = BigramStats::new(2).unwrap();

        stats.observe(&tokens).unwrap();

        assert_eq!(
            stats.probabilities(b),
            Err(BigramStatsError::NoOutgoingTransitions(b))
        );
    }
}
