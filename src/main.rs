use tiktoken_rs::{r50k_base, CoreBPE};

fn encode_data(data: &str, bpe: &CoreBPE) -> Vec<u32> {
    bpe.encode_with_special_tokens(data).to_vec()
}

fn decode_data(tokens: &[u32], bpe: &CoreBPE) -> String {
    bpe.decode(tokens.to_vec()).unwrap()
}

fn main() {
    // Load the data from the file
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    // Initialize the tokenizer
    let bpe = r50k_base().unwrap();

    let encoded_sample = encode_data(&data, &bpe);

    // Pre-allocate a vector for context to avoid repeated allocations
    let mut context = Vec::with_capacity(encoded_sample.len());

    let mut i = 0;
    while i < encoded_sample.len() - 1 {
        // Add the current token to the context
        context.push(encoded_sample[i]);

        // Get the next token as the desired token
        let desired = encoded_sample[i + 1];

        // Decode context and desired token
        let decoded_context = decode_data(&context, &bpe);
        let decoded_desired = decode_data(&[desired], &bpe);

        // Print the context and desired token
        println!("{:?} ---> {}", decoded_context, decoded_desired);

        i += 1;
    }
}
