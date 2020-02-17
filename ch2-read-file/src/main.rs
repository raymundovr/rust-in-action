use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("Readme.md").unwrap();
    let reader = BufReader::new(f);

    for l in reader.lines() {
        let line = l.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
