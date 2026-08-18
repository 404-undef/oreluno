use rustllm::Rng;
use rustllm::{BigramStats, BigramStatsError};
use rustllm::{CharTokenizer, TokenId};
use rustllm::{CliArgs, CliArgsError, usage};
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};

fn main() {
    match CliArgs::parse(env::args().skip(1)) {
        Err(CliArgsError::Usage) => {
            println!("{}", usage());
        }

        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }

        Ok(cli_args) => {
            if let Err(error) = run(cli_args) {
                eprintln!("Error: {error}");
                std::process::exit(1);
            }
        }
    }
}

/// Основная функция, которая выполняет разбор аргументов и обработку текста
fn run(cli_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let mut text = cli_args.text.unwrap_or_default();
    let seed = cli_args.seed.unwrap_or_default();

    if let Some(train_path) = cli_args.train {
        text = fs::read_to_string(&train_path)?;
    }

    let tokenizer = CharTokenizer::from_corpus(&text)?;
    let tokens = tokenizer.encode(&text)?;

    print_corpus_info(&text, &tokenizer, &tokens);

    println!();
    println!("Tokens: {:?}", &tokens[..tokens.len().min(100)]);
    println!("Decoded tokens: {}", tokenizer.decode(&tokens)?);

    let mut stats = BigramStats::new(tokenizer.vocab_size())?;
    stats.observe(&tokens)?;

    println!();
    print_bigram_stats(&tokenizer, &stats)?;

    println!();
    let mut rng = Rng::new(seed);
    println!("seed: {seed}");
    println!("random: {}", rng.next_f64());
    println!("random: {}", rng.next_f64());
    println!("random: {}", rng.next_f64());
    println!("random: {}", rng.next_f64());

    Ok(())
}

fn print_corpus_info(text: &str, tokenizer: &CharTokenizer, tokens: &[TokenId]) {
    println!("Corpus:");
    println!("  bytes: {:>6}", text.len());
    println!("  chars: {:>6}", text.chars().count());
    println!("  tokens: {:>5}", tokens.len());
    println!("  vocabulary: {:>1}", tokenizer.vocab_size());
}

fn print_bigram_stats(
    tokenizer: &CharTokenizer,
    stats: &BigramStats,
) -> Result<(), Box<dyn Error>> {
    println!("Bigram transitions:");

    for current_idx in 0..tokenizer.vocab_size() {
        let current = TokenId::from_index(current_idx)
            .ok_or("failed to convert vocabulary index to TokenId")?;

        println!("'{}' [{current}]", tokenizer.decode(&[current])?);

        match stats.probabilities(current) {
            Ok(probabilities) => {
                for (next_idx, &probability) in probabilities.iter().enumerate() {
                    let next = TokenId::from_index(next_idx)
                        .ok_or("failed to convert vocabulary index to TokenId")?;

                    let count = stats.count(current, next)?;
                    if count == 0 {
                        continue;
                    }

                    println!(
                        "  -> '{}' [{next}]: count={count}, p={probability:.6}",
                        tokenizer.decode(&[next])?
                    );
                }
            }

            Err(BigramStatsError::NoOutgoingTransitions(_)) => {
                println!("  no outgoing transitions");
            }

            Err(error) => return Err(error.into()),
        }
    }

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


    RustLLM Level 0.2 - Statistical Bigram Language Model
    ────────────────────
    Научить RustLLM замечать, какие токены встречаются друг после друга,
    превращать эти наблюдения в вероятности и на их основе предсказывать
    следующий токен

    Подэтап         Реализация                  Математика
    0.2.1       пары соседних токенов       последовательности
    0.2.2       V * V counts                таблицы, индексы
    0.2.3       probabilities               дроби, сумма, вероятность
    0.2.4       sampling                    интервалы [0,1)
    0.2.5       deterministic PRNG          псевдослучайность
    0.2.6       text generation loop        условная вероятность
    0.2.7       CLI + тесты                 интеграция

*/
