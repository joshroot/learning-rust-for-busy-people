use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\n--INFO-- dbg!(&args):");
    dbg!(&args);

    let name_of_binary = &args[0];
    let file_path = &args[1];

    println!("\n--INFO-- Command-line argument values:");
    println!("name_of_binary: {name_of_binary}");
    println!("file_path: {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Could not read the file");

    let first_line = contents.lines().next()
        .expect("File is empty");

    let title = String::from(first_line).replace("# ", "");

    println!("\nTitle: {title}");
}
