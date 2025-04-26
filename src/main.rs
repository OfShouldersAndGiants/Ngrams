mod sampling;

use sampling::{GPTDataset, DataLoader};

fn main() {
    // Load the data from the file into memory
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    // Let's create a dataset with a small context size (4) that matches the book example
    // and where we use a stride of one token
    let dataset = GPTDataset::new(&data, 4, 1).unwrap();

    // Create a dataloader with a batch size of 2 and drop_last set to true
    let mut dataloader = DataLoader::new(dataset, 2, true, None);

    // Get the first batch
    if let Some((input, target)) = dataloader.next_batch() {
        println!("Input token IDs:  {:?}", input);
        println!("Target token IDs: {:?}", target);
    }
}
