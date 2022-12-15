use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    for line_in in reader.lines() {
        let line = line_in.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
