use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

    let input_file = File::open("input")
        .expect("File not found");
    let buf_reader = BufReader::new(input_file);

    let mut last_int: i16 = -1;

    let mut larger_than_last: i16 = 0;

    for line in buf_reader.lines() {
        let this_int: i16 = match line {
            Ok(line_res) => line_res.parse().unwrap(),
            Err(_e) => -1,
        };

        if last_int == -1 || this_int == -1 {
            
        }
        else if this_int > last_int {
            larger_than_last += 1;
        }

        last_int = this_int;
    }

    println!("Measurements greater than the last: {}", larger_than_last);
}
