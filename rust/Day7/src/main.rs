use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn day7_part1(data: &Vec<i32>) -> i32 {
    return 0;
}

fn read_line_file(path_string: &'static str) -> Vec<i32> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let data: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    return data;
}

fn main() {
    println!("AOC2021 Day 6: please god");

    let data = read_line_file("./input_sample.txt");
    let result = day7_part1(&data);
    println!("Day 6 Sample 1: Result: {}", result);

    // let result = day6_part1(&data, 5120);
    // println!("Day 6 Sample 1: Result: {}", result);
}
