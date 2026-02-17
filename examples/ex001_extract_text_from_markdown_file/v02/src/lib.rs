// Library crate

use std::fs;

pub fn run(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Could not read the file");

    let first_line = contents.lines().next()
        .expect("File is empty");

    let title = String::from(first_line).replace("# ", "");

    println!("Title: {title}");
}
