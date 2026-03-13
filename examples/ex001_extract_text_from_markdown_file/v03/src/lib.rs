// Library crate

use std::error::Error;
use std::fs;

pub fn get_file_path(args: &[String]) -> &String {
    if args.len() < 2 {
        panic!("Not enough command-line arguments");
    }
    let file_path = &args[1];

    file_path
}

pub fn run(file_path: &String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let first_line = contents.lines().next().ok_or("File is empty")?;

    let title = String::from(first_line).replace("# ", "");

    println!("Title: {title}");

    Ok(())
}
