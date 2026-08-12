//! Реализует разбор аргументов командной строки для локального импортера

use std::{error::Error, path::PathBuf};

/// Ошибка разбора аргументов локального импортера
#[derive(Debug, PartialEq, Eq)]
pub enum CliArgsError {
    /// Обязательный именованный аргумент не передан
    #[allow(unused)]
    MissingArg(&'static str),
    /// После имени аргумента отсутствует его значение
    MissingValue(&'static str),
    /// Передан неизвестный именованный аргумент
    UnknownArg(String),
    /// Пользователь запросил справку вместо запуска импорта
    Usage,
}

/// Реализуем Error для CliArgsError, чтобы можно было использовать его в Result
impl Error for CliArgsError {}

/// Реализуем Display для CliArgsError, чтобы можно было красиво выводить ошибки пользователю
impl std::fmt::Display for CliArgsError {
    /// Форматирует ошибку разбора аргументов для отображения пользователю
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingArg(arg) => write!(formatter, "Error: missing required argument {arg}"),
            Self::MissingValue(arg) => write!(formatter, "Error: missing value for argument {arg}"),
            Self::UnknownArg(arg) => write!(formatter, "Error: unknown argument {arg}"),
            Self::Usage => write!(formatter, "{}", usage()),
        }
    }
}

/// Структура для хранения разобранных аргументов
#[derive(Debug, PartialEq, Eq)]
pub struct CliArgs {
    pub text: String,
    pub train: PathBuf,
}

// TODO: Переделать на:
// pub struct CliArgs {
//     pub text: Option<String>,
//     pub train: Option<PathBuf>,
// }
// Тогда: None
// явно означает: аргумент не передан

impl CliArgs {
    /// Разбирает аргументы и отклоняет неизвестные ключи
    pub fn parse(raw_args: impl Iterator<Item = String>) -> Result<CliArgs, CliArgsError> {
        let mut text = String::new();
        let mut train = PathBuf::new();
        let mut args = raw_args.peekable();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--text" => text = next_value(&mut args, "text")?,
                "--train" => train = PathBuf::from(next_value(&mut args, "train")?),
                "--help" | "-h" => return Err(CliArgsError::Usage),
                _ => return Err(CliArgsError::UnknownArg(arg)),
            }
        }

        Ok(Self {
            text,
            train,
        })
    }
}

// Получает следующее значение аргумента, если оно существует и не является именованным аргументом
fn next_value(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
    name: &'static str,
) -> Result<String, CliArgsError> {
    let Some(value) = args.next() else {
        return Err(CliArgsError::MissingValue(name));
    };

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
fn usage() -> &'static str {
    "usage: rustllm --text <text> | --train <path>"
}
