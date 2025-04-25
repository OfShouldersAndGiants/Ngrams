mod sampling;

use sampling::GPTDataset;

fn main() {
    // Load the data from the file into memory
    let data = std::fs::read_to_string("src/assets/files/the-verdict.txt").unwrap();

    // Let's create a dataset with a small context size (4) that matches the book example
    // and where we use a stride of one token
    let dataset = GPTDataset::new(&data, 4, 1).unwrap();

    println!("\nFirst sample:");
    if let Some(first_sample) = dataset.get(0) {
        println!("Input token IDs:  {:?}", first_sample.0);
        println!("Target token IDs: {:?}", first_sample.1);
    }

    println!("\nSecond sample:");
    if let Some(second_sample) = dataset.get(1) {
        println!("Input token IDs:  {:?}", second_sample.0);
        println!("Target token IDs: {:?}", second_sample.1);
    }
}
