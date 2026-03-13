// Binary crate

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = ex001_extract_text_from_markdown_file_v03::get_file_path(&args);

    if let Err(e) = ex001_extract_text_from_markdown_file_v03::run(file_path) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
