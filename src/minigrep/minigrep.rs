mod lib;

use std::{env, fs, process};
use std::error::Error;
use crate::lib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(e) = lib::run(config) {
        eprintln!("Error cause : {}", e);
        process::exit(1);
    }
}



