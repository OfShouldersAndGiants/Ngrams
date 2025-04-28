mod ngrams;

use std::env;
use ngrams::ngram_main;

fn main() {
    // Retrieve the arguments from the console input
    let args: Vec<String> = env::args().collect();
    // Let's pass all the arguments and controll of the app to the ngrams main file
    ngram_main(args);
}
