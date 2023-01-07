use std::io::prelude::*;

const BYTES_PRE_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main () {
    println!("Hello, world!");
}"#;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    INPUT.read_to_end(&mut buffer)?;
    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PRE_LINE) {
        print!("[ox{:08x}] ", position_in_input);
        for byte in line {
            print!("{:02x}", byte)
        }
        println!();
        position_in_input += BYTES_PRE_LINE;
    }

    Ok(())
}
