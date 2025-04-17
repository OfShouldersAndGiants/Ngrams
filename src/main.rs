use tiktoken_rs::r50k_base;

fn main() {
    // The first step is to load the data from the file
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    // Let's print the first 100 characters of the data
    println!("{}", &data[0..100]);

    // Instantiate the tokenizer
    let bpe = r50k_base().unwrap();

    // Encode the tokens
    let tokens = bpe.encode_with_special_tokens(&data);

    // Let's print the number of tokens
    println!("{}", tokens.len());

    // Let's print the tokens
    println!("{:?}", tokens);

    // Let's decode the tokens
    let decoded = bpe.decode(tokens).unwrap();

    // Let's print the decoded tokens
    println!("{}", decoded);
}
