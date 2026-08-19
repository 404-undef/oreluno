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
RustLLM

Usage:
    rustllm --text <text> [options]
    rustllm --train <path> [options]

Modes:
    --text <text>
        Run the model with the provided input text

    --train <path>
        Train the model using data from the specified file

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

/// Реализуем Error для CliArgsError, чтобы можно было использовать его в Result
impl Error for CliArgsError {}

/// Реализуем Display для CliArgsError, чтобы можно было красиво выводить ошибки пользователю
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
