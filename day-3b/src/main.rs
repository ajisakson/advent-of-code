use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input_file = File::open("src/input").expect("File not found");
    let buf_reader = BufReader::new(input_file);

    let mut vec = Vec::new();

    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");
    let gamma_rate: u32;
    let epsilon_rate: u32;

    for line in buf_reader.lines() {
        let line_str = line.unwrap();

        // just a check to see make sure the vector is always the appropriate length
        if vec.len() < line_str.chars().count() {
            vec.resize(line_str.chars().count(), (0, 0));
        }

        for (index, my_char) in line_str.chars().enumerate() {
            match my_char {
                '1' => {
                    vec[index].0 += 1;
                }
                '0' => {
                    vec[index].1 += 1;
                }
                _ => {
                    return;
                }
            }
        }
    }

    for bit in vec.iter() {
        let (a, b) = bit;
        if a > b {
            let _ = &gamma_str.push('1');
            let _ = &epsilon_str.push('0');
        } else {
            let _ = &gamma_str.push('0');
            let _ = &epsilon_str.push('1');
        }
    }

    println!("gamma_str = {}, epsilon_str = {}", gamma_str, epsilon_str);

    gamma_rate = u32::from_str_radix(&gamma_str, 2).unwrap();
    epsilon_rate = u32::from_str_radix(&epsilon_str, 2).unwrap();

    println!(
        "gamma_rate = {}, epsilon_rate = {}",
        gamma_rate, epsilon_rate
    );

    let power_consumption = gamma_rate * epsilon_rate;

    println!("The power consumption is {}", power_consumption);
}

// gamma rate is the most common value of a bit in each position
// epsilon rate is the least common bit
// the power consumption is gamma * epsilon represented in decimal
