use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PRE_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage :fview FINENAME");

    let mut f = File::open(&fname).expect("Unable to open file");
    let mut pos = 0;

    let mut buffer = [0; BYTES_PRE_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[ox{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!();
        pos += BYTES_PRE_LINE;
    }
}
