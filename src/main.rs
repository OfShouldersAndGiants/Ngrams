use tiktoken_rs::{r50k_base, Rank};


fn encode_data(data: &str) -> Vec<u32> {
    // Instantiate the tokenizer, we use the r50k_base model because it's the default model for GPT-2
    // and that's the one the book uses
    let bpe = r50k_base().unwrap();

    // Encode the tokens
    let tokens = bpe.encode_with_special_tokens(&data);

    // Return the tokens
    tokens
}

fn decode_data(tokens: Vec<Rank>) -> String {
    let bpe = r50k_base().unwrap();

    let data: String = bpe.decode(tokens).unwrap();

    data
}

fn main() {
    // The first step is to load the data from the file
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();



    let sample: &str = &data[50..data.len()];

    let encoded_sample = encode_data(sample);

    for i in 1..=encoded_sample.len() {
        let context = &encoded_sample[..i];
        let desired = encoded_sample[i];

        // Print the context and desired token
        println!("{:?} ---> {}", decode_data(context.to_vec()), decode_data([desired].to_vec()));
    }

}
