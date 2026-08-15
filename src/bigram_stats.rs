#![allow(unused)]

use crate::TokenId;
use std::error::Error;
use std::fmt;

/*
    TokenId
        ↓
    transition counts
        ↓
    probabilities
*/

#[derive(Debug)]
pub struct BigramStats {
    counts: Vec<u64>, // Плоская матрица
    vocab_size: usize,
}

/*
                                        Для BigramStats:
            Инвариант                                                 Значение
     counts.len() == V * V	                                 матрица имеет правильный размер
     любой используемый TokenId < V	                         токен принадлежит vocabulary
     count никогда не отрицателен                            поэтому u64
     после первого observe(N tokens) сумма counts = N - 1	 каждая соседняя пара учтена
     probability ∈ [0, 1]                                    определение вероятности
     сумма строки probabilities ≈ 1.0                        полное распределение
     probabilities.len() == V                                одна вероятность на каждый следующий токен
      *Последнее ≈ 1.0, а не обязательно строго == 1.0

    Как превратить два индекса в один:
        row = current
        column = next
        V = vocab_size

    Формула:
        index = row * V + column

    Пример:
        row = 2
        column = 3
        V = 4

        index = 2 × 4 + 3
              = 11

*/

impl BigramStats {
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

    pub fn observe(&mut self, tokens: &[TokenId]) -> Result<(), BigramStatsError> {
        /*
            Пусть приходит:
                [A, B, C, D]

            Метод должен обработать:
                (A, B)
                (B, C)
                (C, D)

            И увеличить соответствующие счётчики
            tokens.windows(2)

            Метод не должен обнулять предыдущие counts
        */

        todo!()
    }

    /// Возвращает сколько раз `next` встречается после `current`
    pub fn count(&self, current: TokenId, next: TokenId) -> Result<u64, BigramStatsError> {
        todo!()
    }

    pub fn probabilities(&self, current: TokenId) -> Result<Vec<f64>, BigramStatsError> {
        /*
            Допустим counts для b:
                [3, 1, 0]

            Сумма:
                4

            Результат:
                vec![
                    0.75,
                    0.25,
                    0.0,
                ]

            Длина результата всегда:
                vocab_size

            Особый случай, например:
                abc
            т.е
                c → ???
            Counts:
                [0, 0, 0] -> 0 / 0
            =>
                probabilities(c) -> Err(BigramError::NoOutgoingTransitions(c))
        */
        todo!()
    }

    fn index(&self, current: TokenId, next: TokenId) -> Result<usize, BigramStatsError> {
        /*
           TokenId + TokenId
                   ↓
           проверить границы
                   ↓
               flat index
        */
        todo!()
    }
}

/// Ошибки BigramStats
#[derive(Debug, PartialEq, Eq)]
pub enum BigramStatsError {
    EmptyVocabulary, // Модель без единого допустимого токена не имеет смысла
    MatrixSizeOverflow,
    TokenOutOfVocabulary { token: TokenId, vocab_size: usize },
    CountOverflow { current: TokenId, next: TokenId },
    NoOutgoingTransitions(TokenId),
}

impl Error for BigramStatsError {}

impl fmt::Display for BigramStatsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVocabulary => write!(formatter, "Vocabulary size is empty"),
            Self::MatrixSizeOverflow => {
                write!(formatter, "Vocabulary size too large to fit in `usize`")
            }
            Self::TokenOutOfVocabulary { token, vocab_size } => todo!(),
            Self::CountOverflow { current, next } => todo!(),
            Self::NoOutgoingTransitions(token_id) => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_zeroed_counts() {
        let v: usize = 3;
        let result =
            BigramStats::new(v).expect(&format!("Failed to create BigramStats with size `{v}`"));

        // Начальное состояние правильное
        assert_eq!(result.counts.len(), usize::from(v).checked_mul(v).unwrap());

        // для каждого count:
        //     count == 0
        assert!(result.counts.iter().all(|c| c == &0));
    }

    #[test]
    fn new_rejects_empty_vocabulary() {
        let v: usize = 0;
        let result = BigramStats::new(v);

        // Должен вернуть: EmptyVocabulary
        assert!(
            matches!(result, Err(BigramStatsError::EmptyVocabulary)),
            "Expected Err(BigramStatsError::EmptyVocabulary), but got {:?}",
            result
        );
    }

    /*
                Тест                                Что доказывает
        new_creates_zeroed_counts               начальное состояние правильное
        observe_counts_bigrams                  пары действительно считаются
        observe_accumulates_counts              несколько вызовов складываются
        empty_tokens_do_nothing                 [] корректен
        single_token_does_nothing               одна буква не создаёт bigram
        count_rejects_out_of_vocab_token        соблюдаются границы
        probabilities_are_normalized            вероятность считается правильно
        probabilities_sum_to_one                распределение корректно
        no_outgoing_transitions_returns_error   нет деления 0 / 0

        Исходная строка:
            abba

        Например
            После:
                a → b
                b → b
                b → a

            должно быть:
                count(a, a) = 0
                count(a, b) = 1

                count(b, a) = 1
                count(b, b) = 1

            Вероятности:
                P(a | a) = 0
                P(b | a) = 1

                P(a | b) = 0.5
                P(b | b) = 0.5
    */
}
