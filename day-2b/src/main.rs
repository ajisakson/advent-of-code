use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input_file = File::open("src/input")
        .expect("File not found");
    let buf_reader = BufReader::new(input_file);

    let mut hor_pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;


    for line in buf_reader.lines() {
        let mut direction = String::new();
        let mut magnitude: i32;

        match line {
            Ok(line_res) => {
                let res_arr: Vec<&str> = line_res.split_whitespace().collect();
                direction = res_arr[0].to_owned();
                magnitude = res_arr[1].trim().parse().unwrap();
            },
            Err(_e) => return,
        }

        match (direction.as_ref()) {
            "forward" => {
                hor_pos += magnitude;
                depth += aim*magnitude;
            },
            "down" => {
                aim += magnitude;
            },
            "up" => {
                aim -= magnitude;
            },
            _ => return,
        }
    }

    let product: i32 = depth * hor_pos;

    println!("Final horizontal position * final depth = {}", product);
}