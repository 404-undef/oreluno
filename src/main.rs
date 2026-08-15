use rustllm::BigramStats;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

fn main() {
    // Разбираем аргументы командной строки и обрабатываем текст
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// Основная функция, которая выполняет разбор аргументов и обработку текста
fn run() -> Result<(), Box<dyn Error>> {
    let cli_args = rustllm::CliArgs::parse(env::args().skip(1))?;
    let mut text = cli_args.text.unwrap_or_default();

    if let Some(train_path) = cli_args.train {
        text = fs::read_to_string(&train_path)?;
    }

    let tokenizer = rustllm::CharTokenizer::from_corpus(&text)?;
    let tokens = tokenizer.encode(&text)?;

    println!("Bytes count: {}", text.len());
    println!("Chars count: {}", text.chars().count());
    println!("Tokens count: {}", tokens.len());
    println!("Vocabulary size: {}", tokenizer.vocab_size());
    println!("Tokens: {:?}", &tokens[..tokens.len().min(100)]);
    println!("Source text: {}", tokenizer.decode(&tokens)?);

    let bigram_stats = BigramStats::new(tokenizer.vocab_size())?;
    println!("bigram_stats: {:?}", bigram_stats);

    Ok(())
}

/// Получает строку из stdin и возвращает её
#[allow(dead_code)]
fn text_from_input() -> Result<String, io::Error> {
    let mut input_str = String::new();

    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_str)?;
    Ok(input_str.to_string())
}

/*
    ┌─────────────────────────────────────────────────────────────────────────────┐
    │  Этап   │         Что делаем	                │     Что изучаем             │
    └─────────────────────────────────────────────────────────────────────────────┘
        0.1     Corpus + character tokenizer          tokens, vocabulary, TokenId
        0.2     статистическая Markov/bigram модель   вероятность
        0.3     Матрица весов                         parameters, logits
        0.4     Softmax + Cross Entropy               probability, loss
        0.5     Ручной gradient descent               gradient, learning rate
        0.6     Training + validation                 обучение модели
        0.7     Generation + checkpoint               inference, sampling
    └─────────────────────────────────────────────────────────────────────────────┘


    RustLLM Level 0.1
    ────────────────────
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


    "Привет"
        │
        ▼
    CharTokenizer
        │
        ▼
    [TokenId(...), TokenId(...), ...]



    RustLLM Level 0.2 - Statistical Bigram Language Model
    ────────────────────
    Научить RustLLM замечать, какие токены встречаются друг после друга,
    превращать эти наблюдения в вероятности и на их основе предсказывать
    следующий токен


        ┌─────────────────────┐
        │   обучающий текст   │
        └──────────┬──────────┘
                   │
                   ▼
             CharTokenizer
                   │
                   ▼
           [4, 8, 7, 5, 6, 9]
                   │
                   ▼
           StatisticalBigram
                   │
        ┌──────────┴─────────────┐
        ▼                        ▼
 transition counts         probabilities
        │                        │
        └──────────┬─────────────┘
                   ▼
           следующий TokenId
                   │
                   ▼
             CharTokenizer
                   │
                   ▼
                 текст

    Подэтап         Реализация                  Математика
    0.2.1       пары соседних токенов       последовательности
    0.2.2       V × V counts                таблицы, индексы
    0.2.3       probabilities               дроби, сумма, вероятность
    0.2.4       sampling                    интервалы [0,1)
    0.2.5       deterministic PRNG          псевдослучайность
    0.2.6       text generation loop        условная вероятность
    0.2.7       CLI + тесты                 интеграция

*/
