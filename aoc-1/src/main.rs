// Fuel required to launch a given module is based on its mass. 
// Specifically, to find the fuel required for a module, 
// take its mass, divide by three, round down, and subtract 2.

use std::env;
use std::error::Error;
use std::process;
use dayone::Config;

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
