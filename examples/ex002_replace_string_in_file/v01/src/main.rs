use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\n--INFO-- dbg!(&args):");
    dbg!(&args);

    let name_of_binary = &args[0];
    let file_path = &args[1];
    let orig_string = &args[2];
    let new_string = &args[3];

    println!("\n--INFO-- Command-line argument values:");
    println!("name_of_binary: {name_of_binary}");
    println!("file_path: {file_path}");
    println!("orig_string: {orig_string}");
    println!("new_string: {new_string}");

    let orig_contents = fs::read_to_string(file_path)
        .expect("Could not read the file");

    let new_contents = String::from(orig_contents).replace(orig_string, new_string);

    println!("\n--INFO-- New file contents:\n{new_contents}");
}
