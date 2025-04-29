mod ngrams;

use std::env;
use ngrams::ngram_main;

fn main() {
    // Retrieve the arguments from the console input and pass them to the ngrams main file
    // Remember that you need to pass something like this:
    // cargo run -- -i "when going in the mou" -m trigram
    // And the program will output a suggestion for the incomplete word: 'mountains'
    ngram_main(env::args().collect());
}
