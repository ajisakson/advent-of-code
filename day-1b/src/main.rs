use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input_file = File::open("src/input")
        .expect("File not found");
    let buf_reader = BufReader::new(input_file);

    let mut last_sum: i16 = -1;

    let mut larger_than_last: i16 = 0;

    let mut line0: i16 = -1;
    let mut line1: i16 = -1;
    let mut line2;

    for line in buf_reader.lines() {
        line2 = line1;
        line1 = line0;
        line0 = match line {
            Ok(line_res) => line_res.trim().parse().unwrap(),
            Err(_e) => -1,
        };
        
        if line0 > -1 && line1 > -1 && line2 > -1{
            let this_sum = line0 + line1 + line2;
            if last_sum == -1 || this_sum == -1 {
            }
            else if this_sum > last_sum {
                larger_than_last += 1;
            }               
            last_sum = this_sum;
        }
    }

    println!("Measurements greater than the last: {}", larger_than_last);
}