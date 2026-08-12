mod args;
mod tokenizer;

use std::env;
use std::error::Error;
use std::io::{self, Write};

fn main() {
    // Разбираем аргументы командной строки и обрабатываем текст
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// Основная функция, которая выполняет разбор аргументов и обработку текста
fn run() -> Result<(), Box<dyn Error>> {
    let cli_args = args::CliArgs::parse(env::args().skip(1))?;
    let mut text = cli_args.text.clone();

    if !cli_args.train.as_os_str().is_empty() {
        text = std::fs::read_to_string(&cli_args.train)?;
    }

    let tokenizer = tokenizer::CharTokenizer::from_corpus(&text)?;
    let tokens = tokenizer.encode(&text)?;

    println!("Bytes count: {}", text.len());
    println!("Chars count: {}", text.chars().count());
    println!("Tokens count: {}", tokens.len());
    println!("Vocabulary size: {}", tokenizer.vocab_size());
    println!("Tokens: {:?}", &tokens[..tokens.len().min(100)]);
    println!("Source text: {}", tokenizer.decode(&tokens)?);

    Ok(())
}

/// Получает строку из stdin и возвращает её
#[allow(dead_code)]
fn text_from_input() -> String {
    let mut input_str = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    input_str.to_string()
}

/*
    Этап    Что делаем	                    Что изучаем
    0.1     Corpus + character tokenizer    tokens, vocabulary, TokenId
    0.2     Статистический bigram           вероятность
    0.3     Матрица весов                   parameters, logits
    0.4     Softmax + Cross Entropy         probability, loss
    0.5     Ручной gradient descent         gradient, learning rate
    0.6     Training + validation           обучение модели
    0.7     Generation + checkpoint         inference, sampling
*/

/*
    RustLLM Level 0.1

    1. cargo new rustllm
    2. положить небольшой corpus.txt
    3. прочитать его в Rust
    4. преобразовать UTF-8 -> bytes
    5. вывести:
        - размер текста
        - количество токенов
        - первые 100 token IDs
    6. восстановить текст обратно
    7. проверить encode -> decode
*/
