use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn day1(input_data: &Vec<i32>, window_size: u32) -> i32 {
    let mut prev_value: i32 = i32::MAX;
    let mut increases: i32 = 0;
    let mut current_value: i32;
    for index in 0..(input_data.len() as u32-(window_size-1)) {
        current_value = 0;
        for window_index in 0..window_size {
            current_value += input_data[(index + window_index) as usize];
        }
        if current_value > prev_value {
            increases += 1;
        }
        prev_value = current_value;
    }
    return increases;
}

fn read_int_file(path_string: &'static str) -> Vec<i32> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<i32> = vec![];
    for line in lines {
        // println!("{}", line);
        let parse_val: i32 = line.parse::<i32>().unwrap();
        data.push(parse_val);
    }
    return data;
}

fn main() {
    println!("AOC2021 Day 1 aaa fuck rust");
    let mut result: i32;
    let data: Vec<i32> = read_int_file("./input.txt");
    result = day1(&data, 1);
    println!("Problem 1: {}", result);
    result = day1(&data, 3);
    println!("Problem 2: {}", result);
}