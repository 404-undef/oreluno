//! Реализует разбор аргументов командной строки

use std::error::Error;
use std::path::PathBuf;

/// Структура для хранения разобранных аргументов
#[derive(Debug, PartialEq, Eq)]
pub struct CliArgs {
    pub text: Option<String>,
    pub train: Option<PathBuf>,
    pub seed: Option<u64>,
    pub length: Option<usize>,
}

impl CliArgs {
    /// Разбирает аргументы и отклоняет неизвестные ключи
    pub fn parse(raw_args: impl Iterator<Item = String>) -> Result<CliArgs, CliArgsError> {
        let mut text = None;
        let mut train = None;
        let mut seed = None;
        let mut length = None;
        let mut args = raw_args.peekable();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--text" => {
                    text = Some(
                        args.next_if(|value| !value.starts_with("--"))
                            .unwrap_or_default(),
                    );
                }
                "--train" => {
                    let value = next_value(&mut args, "train")?;

                    if !value.is_empty() {
                        train = Some(PathBuf::from(value));
                    }
                }
                "--seed" => {
                    let value = next_value(&mut args, "seed")?;

                    let parsed_seed =
                        value
                            .parse::<u64>()
                            .map_err(|_| CliArgsError::InvalidValue {
                                arg: "seed",
                                value,
                                expected: "u64",
                            })?;

                    seed = Some(parsed_seed);
                }
                "--length" => {
                    let value = next_value(&mut args, "length")?;

                    let parsed_length =
                        value
                            .parse::<usize>()
                            .map_err(|_| CliArgsError::InvalidValue {
                                arg: "length",
                                value,
                                expected: "usize",
                            })?;

                    length = Some(parsed_length);
                }
                "--help" | "-h" => return Err(CliArgsError::Usage),
                _ => return Err(CliArgsError::UnknownArg(arg)),
            }
        }

        Ok(Self {
            text,
            train,
            seed,
            length,
        })
    }
}

// Получает следующее значение аргумента, если оно существует и не является именованным аргументом
fn next_value(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
    name: &'static str,
) -> Result<String, CliArgsError> {
    let value = args.next().ok_or(CliArgsError::MissingValue(name))?;

    if value.starts_with("--") {
        return Err(CliArgsError::MissingValue(name));
    }

    Ok(value)
}

// Проверяет, что обязательный аргумент был передан, иначе возвращает ошибку
#[allow(dead_code)]
fn required_arg<T>(value: Option<T>, name: &'static str) -> Result<T, CliArgsError> {
    value.ok_or(CliArgsError::MissingArg(name))
}

// Возвращает строку с инструкцией по использованию программы
pub fn usage() -> &'static str {
    "\
Oreluno

Usage:
    oreluno --text <text> [options]
    oreluno --train <path> [options]

Modes:
    --text <text>
        Run the model with the provided input text

    --train <path>
        Train the model using data from the specified file
        Takes precedence over --text when both are provided

Options:
    --seed <u64>
        Seed for the pseudorandom number generator
        Default: 0

    --length <usize>
        Number of tokens to generate.
        Default: 0

    -h, --help
        Show this help message
"
}

/// Ошибка разбора аргументов командной строки
#[derive(Debug, PartialEq, Eq)]
pub enum CliArgsError {
    InvalidValue {
        value: String,
        arg: &'static str,
        expected: &'static str,
    },

    /// Обязательный именованный аргумент не передан
    MissingArg(&'static str),

    /// После имени аргумента отсутствует его значение
    MissingValue(&'static str),

    /// Передан неизвестный именованный аргумент
    UnknownArg(String),

    /// Пользователь запросил справку вместо запуска
    Usage,
}

/// Реализует `Error`, чтобы `CliArgsError` можно было возвращать через `Result`
impl Error for CliArgsError {}

/// Реализует `Display` для понятного вывода ошибок
impl std::fmt::Display for CliArgsError {
    /// Форматирует ошибку разбора аргументов для отображения пользователю
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue {
                value,
                arg,
                expected,
            } => {
                write!(
                    formatter,
                    "invalid value `{value}` for argument `{arg}`: expected {expected}"
                )
            }
            Self::MissingArg(arg) => {
                write!(formatter, "missing required argument `{arg}`")
            }
            Self::MissingValue(arg) => {
                write!(formatter, "missing value for argument `{arg}`")
            }
            Self::UnknownArg(arg) => {
                write!(formatter, "unknown argument `{arg}`")
            }
            Self::Usage => {
                write!(formatter, "{}", usage())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<CliArgs, CliArgsError> {
        CliArgs::parse(args.iter().map(|arg| arg.to_string()))
    }

    #[test]
    fn parses_text() {
        let args = parse(&["--text", "abba"]).unwrap();

        assert_eq!(args.text, Some("abba".to_string()));
        assert_eq!(args.train, None);
    }

    #[test]
    fn parses_empty_text() {
        let args = parse(&["--text"]).unwrap();

        assert_eq!(args.text, Some(String::new()));
    }

    #[test]
    fn parses_train_path() {
        let args = parse(&["--train", "data/corpus.txt"]).unwrap();

        assert_eq!(args.train, Some(PathBuf::from("data/corpus.txt")));
    }

    #[test]
    fn parses_seed() {
        let args = parse(&["--seed", "12345"]).unwrap();

        assert_eq!(args.seed, Some(12345));
    }

    #[test]
    fn parses_length() {
        let args = parse(&["--length", "32"]).unwrap();

        assert_eq!(args.length, Some(32));
    }

    #[test]
    fn accepts_zero_length() {
        let args = parse(&["--length", "0"]).unwrap();

        assert_eq!(args.length, Some(0));
    }

    #[test]
    fn parses_all_options_together() {
        let args = parse(&[
            "--text",
            "abba",
            "--train",
            "data/corpus.txt",
            "--seed",
            "42",
            "--length",
            "10",
        ])
        .unwrap();

        assert_eq!(args.text, Some("abba".to_string()));
        assert_eq!(args.train, Some(PathBuf::from("data/corpus.txt")));
        assert_eq!(args.seed, Some(42));
        assert_eq!(args.length, Some(10));
    }

    #[test]
    fn rejects_invalid_seed() {
        let result = parse(&["--seed", "abc"]);

        assert_eq!(
            result,
            Err(CliArgsError::InvalidValue {
                arg: "seed",
                value: "abc".to_string(),
                expected: "u64",
            })
        );
    }

    #[test]
    fn rejects_invalid_length() {
        let result = parse(&["--length", "abc"]);

        assert_eq!(
            result,
            Err(CliArgsError::InvalidValue {
                arg: "length",
                value: "abc".to_string(),
                expected: "usize",
            })
        );
    }

    #[test]
    fn rejects_missing_seed_value() {
        assert_eq!(parse(&["--seed"]), Err(CliArgsError::MissingValue("seed")));
    }

    #[test]
    fn rejects_missing_length_value() {
        assert_eq!(
            parse(&["--length"]),
            Err(CliArgsError::MissingValue("length"))
        );
    }

    #[test]
    fn rejects_unknown_argument() {
        assert_eq!(
            parse(&["--unknown"]),
            Err(CliArgsError::UnknownArg("--unknown".to_string()))
        );
    }

    #[test]
    fn help_returns_usage() {
        assert_eq!(parse(&["--help"]), Err(CliArgsError::Usage));
        assert_eq!(parse(&["-h"]), Err(CliArgsError::Usage));
    }
}
