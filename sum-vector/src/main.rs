// Read in input.txt and sum it up

use dayone::Config;
use std::env;
use std::error::Error;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = dayone::run(config) {
        println!("An exception occurred, {}", e);
        process::exit(1);
    };
}
