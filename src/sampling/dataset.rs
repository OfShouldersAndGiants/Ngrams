use anyhow::Result;
use tch::{Device, Tensor};
use tiktoken_rs::r50k_base;

pub struct GPTDataset {
    input_ids: Vec<Tensor>,
    target_ids: Vec<Tensor>,
}

impl GPTDataset {
    pub fn new(text: &str, max_length: usize, stride: usize) -> Result<Self> {
        let bpe = r50k_base()?;
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
        if idx < self.len() {
            Some((self.input_ids[idx].copy(), self.target_ids[idx].copy()))
        } else {
            None
        }
    }
}
