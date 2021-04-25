use std::error::Error;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let path = Path::new(filename);
    let display = path.display();

    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}",
                           display, why.description()),
        Ok(file) => file,
    };

    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("> {}", line.unwrap());
    }
}
