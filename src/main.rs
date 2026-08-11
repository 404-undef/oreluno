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
    let mut text = String::new();


    if !cli_args.train.as_os_str().is_empty() {
        text = std::fs::read_to_string(&cli_args.train)?;
    } else {
        todo!();
    }


    // println!("Text length: {}", text.len());
    // println!("Tokens count: {}", tokens.len());
    // println!(
    //     "Tokens: {:?}",
    //     if tokens.len() >= 100 {
    //         tokens[..101].to_vec()
    //     } else {
    //         tokens[..tokens.len()].to_vec()
    //     }
    // );

    // println!("Source text: {}", tokenizer::decode(&tokens));

    Ok(())
}

/// Получает строку из stdin, обрезает пробелы и возвращает её
#[allow(dead_code)]
fn text_from_input() -> String {
    let mut input_str = String::new();

    print!("> ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    input_str.trim().to_string()
}

/*
    Этап    Что делаем	                Что изучаем
    0.1     Corpus + byte               tokenizer tokens, vocabulary
    0.2     Статистический bigram	    вероятность
    0.3     Матрица весов               parameters, logits
    0.4     Softmax + Cross Entropy     probability, loss
    0.5     Ручной gradient descent     gradient, learning rate
    0.6     Training + validation       обучение модели
    0.7     Generation + checkpoint     inference, sampling
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
