use std::fs;

fn main() {
    let file_path = "../data/musical_instruments.md";

    let contents = fs::read_to_string(file_path)
        .expect("Could not read the file");

    let first_line = contents.lines().next()
        .expect("File is empty");

    let title = String::from(first_line).replace("# ", "");

    println!("\nTitle: {title}");
}
