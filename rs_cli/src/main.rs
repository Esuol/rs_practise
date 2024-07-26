fn main() {
    let source_file = std::env::args().nth(1).expect("Except source file path");
    let dest_file = std::env::args().nth(2).expect("Except dest file path");

    println!("Copying {} to {}", source_file, dest_file);
    std::fs::copy(source_file, dest_file).unwrap();
}
