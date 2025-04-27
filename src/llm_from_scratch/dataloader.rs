use tch::Tensor;
use rand::seq::SliceRandom;
use rand;

use crate::llm_from_scratch::GPTDataset;

pub struct DataLoader {
    // The dataset you are reading samples from.
    dataset: GPTDataset,
    // How many samples per batch.
    batch_size: usize,
    // A list of sample indices to load (shuffle if needed).
    indices: Vec<usize>,
    // Where you are currently inside the indices list.
    current_index: usize,
    // Whether to drop the last batch if it’s smaller than batch_size.
    drop_last: bool
}

impl DataLoader {
    pub fn new(dataset: GPTDataset, batch_size: usize, drop_last: bool, shuffle: Option<bool>) -> Self {
        let mut indices: Vec<usize> = (0..dataset.len()).collect();

        // If shuffle is true, shuffle the indices to prevent overfit, adding bias and improve generalization
        if let Some(shuffle) = shuffle {
            if shuffle {
                indices.shuffle(&mut rand::rng());
            }
        }

        Self {
            dataset,
            batch_size,
            indices,
            current_index: 0,
            drop_last
        }

    }

    pub fn next_batch(&mut self) -> Option<(Tensor, Tensor)> {
        // Check if we have reached the end of the dataset
        if self.current_index >= self.indices.len() {
            return None;
        }

        // Calculate the end index for the batch, ensuring we don't exceed dataset length
        // Use min to avoid going out of bounds if the remaining samples are less than batch_size
        let end_idx: usize = (self.current_index + self.batch_size).min(self.indices.len());
        // Get the indices for the current batch
        let batch_indices: &[usize] = &self.indices[self.current_index..end_idx];

        // If drop_last is true and the batch is smaller than batch_size, skip this incomplete batch
        if self.drop_last && (batch_indices.len() < self.batch_size) {
            return None;
        }

        let mut input_tensors: Vec<Tensor> = Vec::new();
        let mut target_tensors: Vec<Tensor> = Vec::new();

        // Colelct input and target tensor for each index in the batch
        for &index in batch_indices {
            if let Some((input, target)) = self.dataset.get(index) {
                input_tensors.push(input);
                target_tensors.push(target);
            }
        }

        // Move the current index forward by batch_size for the next batch call
        self.current_index += self.batch_size;

        // Stack all input tensors and target tensors along dimension 0 to create batched tensors.
        // Why using a 0 dimension? Because we want to stack the tensors along the batch dimension.
        let input_batch = Tensor::stack(&input_tensors, 0);
        let target_batch = Tensor::stack(&target_tensors, 0);

        // Return the batched tensors
        Some((input_batch, target_batch))
    }
}
