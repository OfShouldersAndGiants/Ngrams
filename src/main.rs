mod sampling;

use sampling::dataset::GPTDataset;

fn main() {
    // Load the data from the file
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    let dataset = GPTDataset::new(&data, 1024, 1024).unwrap();

    println!("Dataset length: {}", dataset.len());
    println!("Dataset sample: {:?}", dataset.get(0));
    let sample = dataset.get(0).unwrap();
    println!("Sample input: {:?}", sample.0);
    println!("Sample target: {:?}", sample.1);
}
