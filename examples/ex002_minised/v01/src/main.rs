// Binary crate

use std::env;
use std::process;

use ex002_minised_v01::{Config, Substitution};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Argument parsing error: {err}");
        process::exit(1);
    });

    let sub = Substitution::build(&config.script).unwrap_or_else(|err| {
        eprintln!("minised script parsing error: {err}");
        process::exit(1);
    });

    if let Err(e) = ex002_minised_v01::run(config, sub) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
