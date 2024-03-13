use std::{env, process};
use crate::config::Config;

mod config;

fn main() {

    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
}