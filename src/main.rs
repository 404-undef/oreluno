mod cli;

use cli::{CliArgs, CliArgsError, usage};
use oreluno::bigram::{BigramStats, BigramStatsError};
use oreluno::model::BigramModel;
use oreluno::rng::Rng;
use oreluno::tokenizer::{CharTokenizer, TokenId};
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

/// Запускает основной конвейер языковой модели с готовыми аргументами командной строки
fn run(cli_args: CliArgs) -> Result<(), Box<dyn Error>> {
    let text = match (cli_args.train, cli_args.text) {
        (Some(path), _) => fs::read_to_string(path)?,
        (None, Some(text)) => text,
        (None, None) => String::new(),
    };
    let seed = cli_args.seed.unwrap_or_default();
    let length = cli_args.length.unwrap_or(0);

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

    let model = BigramModel::new(stats);
    let mut rng = Rng::new(seed);

    if length > 0 {
        println!();

        let start = *tokens
            .last()
            .ok_or("cannot generate from an empty token sequence")?;

        let generated_tokens = model.generate(start, length, &mut rng)?;
        let generated = tokenizer.decode(&generated_tokens)?;

        println!("Generated: `{generated}`");
    }

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

/// Читает строку из стандартного потока ввода
#[allow(dead_code)]
fn text_from_input() -> Result<String, io::Error> {
    let mut input_str = String::new();

    print!("> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_str)?;
    Ok(input_str.to_string())
}
