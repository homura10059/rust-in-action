use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];
    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap();
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn main() {
    println!("Hello, world!");
}
