// Binary crate

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let name_of_binary = &args[0];
    let file_path = &args[1];
    dbg!(&name_of_binary);
    dbg!(&file_path);

    ex001_extract_text_from_markdown_file_v02::run(file_path);
}
