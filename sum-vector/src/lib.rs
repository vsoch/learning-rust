// Read in input.txt and sum it up

use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Please provide a filename!");
        }

        let filename = args[1].clone();
        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // read the file
    let vec = read(&config.filename)?;
    let sum: i64 = vec.iter().sum();
    println!("The sum is {:?}", sum);
    Ok(())
}


fn read(path: &str) -> Result<Vec<i64>, io::Error> {
    let file = fs::File::open(path)?;
    // buffered reader uses 4 KB buffer internally
    let br = io::BufReader::new(file);
    let mut v = Vec::new();
    // create an iterator over lines in the reader
    for line in br.lines() {
        let line = line?;
        let n = line   
            .trim() 
            .parse() // parse integer
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        v.push(n);
    }
    Ok(v)
}
