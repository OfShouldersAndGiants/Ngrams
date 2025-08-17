use anyhow::Result;
use tch::Tensor;
use tiktoken_rs::r50k_base;

pub struct GPTDataset {
    input_ids: Vec<Tensor>,
    target_ids: Vec<Tensor>,
}

impl GPTDataset {
    // Create a new dataset from a text file
    // The params are the text, the max length of the input, and the stride
    pub fn new(text: &str, max_length: usize, stride: usize) -> Result<Self> {
        // We are going to use the tiktoken library to tokenize the text. This is a byte level wich avoids wird Unicode edge cases.
        // There are 50k tokens in the base model.
        let bpe = r50k_base()?;
        // We are going to use the encode_with_special_tokens method to tokenize the text
        let token_ids = bpe.encode_with_special_tokens(text);

        let mut input_ids = Vec::new();
        let mut target_ids = Vec::new();

        for i in (0..token_ids.len() - max_length).step_by(stride) {
            let input_chunk: Vec<i64> = token_ids[i..i + max_length].iter().map(|&x| x as i64).collect();
            let target_chunk: Vec<i64> = token_ids[i + 1..i + max_length + 1].iter().map(|&x| x as i64).collect();

            input_ids.push(Tensor::from_slice(&input_chunk));
            target_ids.push(Tensor::from_slice(&target_chunk));
        }

        Ok(Self {
            input_ids,
            target_ids,
        })
    }

    pub fn len(&self) -> usize {
        self.input_ids.len()
    }

    pub fn get(&self, idx: usize) -> Option<(Tensor, Tensor)> {
        if idx >= self.len() { return None; }
        Some((self.input_ids[idx].copy(), self.target_ids[idx].copy()))
    }
}
