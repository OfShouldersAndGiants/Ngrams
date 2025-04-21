mod sampling;

use sampling::dataset::GPTDataset;

fn main() {
    // Load the data from the file
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    let dataset = GPTDataset::new(&data, 1024, 1024).unwrap();
}
