use clap::Parser;
use std::fs;
use super::n_gram_model::NGramModel;

#[derive(Parser)]
struct InputArgs {
    #[clap(short, long, default_value = "src/ngrams/files/mountaineering.txt")]
    filename: String,
    #[clap(short, long)]
    input_text: String,
    #[clap(short, long, value_parser = ["unigram", "bigram", "trigram"])]
    mode: String
}

/// Print suggestion nicely
fn print_suggestion(suggestion: &str, occurrences: usize) {
    println!("{}\t{}", suggestion, occurrences);
}

pub fn main(args: Vec<String>) {
    // Parse the arguments
    let input_args = InputArgs::parse_from(args);

    // Read file
    let corpora: String = fs::read_to_string(input_args.filename).unwrap();

    // Train the model
    // This model contains unigram, bigram, and trigram counts for the corpora
    let model: NGramModel = NGramModel::train(&corpora);

    // Generate suggestions from the model
    match input_args.mode.as_str() {
        "unigram" => {
            let suggestions = model.suggest_unigram(&input_args.input_text);
            print_suggestion(&suggestions.0, suggestions.1);
        }
        "bigram" => {
            let suggestions = model.suggest_bigram(&input_args.input_text);
            print_suggestion(&suggestions.0, suggestions.1);
        }
        "trigram" => {
            let suggestions = model.suggest_trigram(&input_args.input_text);
            print_suggestion(&suggestions.0, suggestions.1);
        }
        _ => {
            eprintln!("Unknown mode: {}", input_args.mode);
            std::process::exit(1);
        }
    }
}
