use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn day2_part1(data: Vec<(String, i32)>) -> i32 {
    let mut horiz: i32 = 0;
    let mut depth: i32 = 0;
    for line in data {
        // line.0 is direction
        // line.1 is distance
        let direction: String = line.0;
        if direction.eq("up") {
            depth -= line.1;
        }
        else if direction.eq("down") {
            depth += line.1;
        }
        else if direction.eq("forward") {
            horiz += line.1;
        }
    }
    return depth * horiz;
}

fn day2_part2(data: Vec<(String, i32)>) -> i32 {
    let mut horiz: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for line in data {
        // line.0 is direction
        // line.1 is distance
        let direction: String = line.0;
        if direction.eq("up") {
            aim -= line.1;
        }
        else if direction.eq("down") {
            aim += line.1;
        }
        else if direction.eq("forward") {
            horiz += line.1;
            depth += aim*line.1;
        }
    }
    return depth * horiz;
}

fn read_diving_file(path_string: &'static str) -> Vec<(String, i32)> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<(String, i32)> = vec![];
    for line in lines {
        // println!("{}", line);
        let split_line = &line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let direction: &String = &split_line[0];
        let distance: i32 = split_line[1].parse::<i32>().unwrap();
        let parse_val: (String, i32) = (direction.to_string(), distance);

        data.push(parse_val);
    }
    return data;
}

fn main() {
    println!("AOC2021 Day 2 unwrap() everything");
    let mut result: i32;

    let data: Vec<(String, i32)> = read_diving_file("./input_sample.txt");
    result = day2_part1(data);
    println!("Sample: {}", result);

    let data: Vec<(String, i32)> = read_diving_file("./input.txt");
    result = day2_part1(data);
    println!("Problem 1: {}", result);

    let data: Vec<(String, i32)> = read_diving_file("./input_sample.txt");
    result = day2_part2(data);
    println!("Part 2 Sample: {}", result);

    let data: Vec<(String, i32)> = read_diving_file("./input.txt");
    result = day2_part2(data);
    println!("Part 2: {}", result);
}