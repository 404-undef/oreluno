use std::error::Error;
use std::fmt;

const EPSILON: f64 = 1e-12;

/// Выбирает индекс из дискретного распределения вероятностей
///
/// `random` должен находиться в диапазоне `[0.0, 1.0)`
/// Выбор выполняется по накопленной сумме вероятностей
///
/// # Errors
///
/// Возвращает ошибку, если:
/// - распределение пусто
/// - `random` находится вне диапазона `[0.0, 1.0)`
/// - распределение содержит недопустимую вероятность
/// - сумма вероятностей отличается от `1.0` больше допустимой погрешности
pub fn sample_index(probabilities: &[f64], random: f64) -> Result<usize, SamplingError> {
    if probabilities.is_empty() {
        return Err(SamplingError::EmptyDistribution);
    }

    if !(0.0..1.0).contains(&random) {
        return Err(SamplingError::InvalidRandomValue(random));
    }

    for (index, &value) in probabilities.iter().enumerate() {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(SamplingError::InvalidProbability { index, value });
        }
    }

    let sum: f64 = probabilities.iter().sum();
    if (sum - 1.0).abs() >= EPSILON {
        return Err(SamplingError::InvalidProbabilitySum(sum));
    }

    let mut cumulative = 0.0;
    for (index, value) in probabilities.iter().enumerate() {
        cumulative += value;
        if random < cumulative {
            return Ok(index);
        }
    }

    Ok(probabilities.len() - 1)
}

/// Ошибки, возникающие при выборке из распределения вероятностей.
#[derive(Debug, PartialEq)]
pub enum SamplingError {
    /// Элемент распределения содержит недопустимую вероятность
    InvalidProbability { index: usize, value: f64 },

    /// Случайное значение находится вне диапазона `[0.0, 1.0)`
    InvalidRandomValue(f64),

    /// Сумма вероятностей отличается от `1.0` больше допустимой погрешности
    InvalidProbabilitySum(f64),

    /// Распределение вероятностей пусто
    EmptyDistribution,
}

impl Error for SamplingError {}

impl fmt::Display for SamplingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDistribution => {
                write!(formatter, "Probability distribution must not be empty")
            }

            Self::InvalidRandomValue(value) => {
                write!(
                    formatter,
                    "Random value `{value}` must be in the range [0.0, 1.0)"
                )
            }

            Self::InvalidProbability { index, value } => {
                write!(
                    formatter,
                    "Invalid probability `{value}` at index `{index}`"
                )
            }

            Self::InvalidProbabilitySum(sum) => {
                write!(
                    formatter,
                    "Probability distribution must sum to 1.0, got `{sum}`"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_index_selects_expected_intervals() {
        let probabilities = [0.2, 0.3, 0.5];

        assert_eq!(sample_index(&probabilities, 0.13).unwrap(), 0);
        assert_eq!(sample_index(&probabilities, 0.37).unwrap(), 1);
        assert_eq!(sample_index(&probabilities, 0.82).unwrap(), 2);
    }

    #[test]
    fn sample_index_handles_interval_boundaries() {
        let probabilities = [0.2, 0.3, 0.5];

        assert_eq!(sample_index(&probabilities, 0.0).unwrap(), 0);
        assert_eq!(sample_index(&probabilities, 0.199999999).unwrap(), 0);

        assert_eq!(sample_index(&probabilities, 0.2).unwrap(), 1);
        assert_eq!(sample_index(&probabilities, 0.499999999).unwrap(), 1);

        assert_eq!(sample_index(&probabilities, 0.5).unwrap(), 2);
        assert_eq!(sample_index(&probabilities, 0.999999999).unwrap(), 2);
    }

    #[test]
    fn sample_index_skips_zero_probability() {
        let probabilities = [0.0, 0.4, 0.6];

        assert_eq!(sample_index(&probabilities, 0.0).unwrap(), 1);
        assert_eq!(sample_index(&probabilities, 0.399999999).unwrap(), 1);
        assert_eq!(sample_index(&probabilities, 0.4).unwrap(), 2);
    }

    #[test]
    fn sample_index_rejects_empty_distribution() {
        assert_eq!(
            sample_index(&[], 0.5),
            Err(SamplingError::EmptyDistribution)
        );
    }

    #[test]
    fn sample_index_rejects_random_below_zero() {
        assert_eq!(
            sample_index(&[1.0], -0.1),
            Err(SamplingError::InvalidRandomValue(-0.1))
        );
    }

    #[test]
    fn sample_index_rejects_random_equal_to_one() {
        assert_eq!(
            sample_index(&[1.0], 1.0),
            Err(SamplingError::InvalidRandomValue(1.0))
        );
    }

    #[test]
    fn sample_index_rejects_random_above_one() {
        assert_eq!(
            sample_index(&[1.0], 1.1),
            Err(SamplingError::InvalidRandomValue(1.1))
        );
    }

    #[test]
    fn sample_index_rejects_negative_probability() {
        let probabilities = [-0.1, 0.6, 0.5];

        assert_eq!(
            sample_index(&probabilities, 0.5),
            Err(SamplingError::InvalidProbability {
                index: 0,
                value: -0.1,
            })
        );
    }

    #[test]
    fn sample_index_rejects_nan_probability() {
        let probabilities = [f64::NAN, 1.0];

        assert!(matches!(
            sample_index(&probabilities, 0.5),
            Err(SamplingError::InvalidProbability {
                index: 0,
                value
            }) if value.is_nan()
        ));
    }

    #[test]
    fn sample_index_rejects_infinite_probability() {
        let probabilities = [f64::INFINITY, 0.0];

        assert_eq!(
            sample_index(&probabilities, 0.5),
            Err(SamplingError::InvalidProbability {
                index: 0,
                value: f64::INFINITY,
            })
        );
    }

    #[test]
    fn sample_index_rejects_invalid_probability_sum() {
        let probabilities = [0.2, 0.3, 0.4];

        assert!(matches!(
            sample_index(&probabilities, 0.5),
            Err(SamplingError::InvalidProbabilitySum(sum))
                if (sum - 0.9).abs() < EPSILON
        ));
    }
}
