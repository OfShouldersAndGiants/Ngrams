fn main() {
    // The first step is to load the data from the file
    let data = std::fs::read_to_string("src/assets/the-verdict.txt").unwrap();

    // Let's print the first 100 characters of the data
    println!("{}", &data[0..100]);
}
