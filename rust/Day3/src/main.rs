use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn day3_part1(data: &Vec<String>) -> i32 {
    let mut gamma_rate_string = "".to_string();
    let mut epsilon_rate_string = "".to_string();
    let string_length = data[0].len();
    // let mut zeros_count: Vec<i32>; 
    // let mut ones_count: Vec<i32>;
    let (zeros_count, ones_count) = get_ones_and_zeros_count(&data);
    for i in 0..string_length {
        // println!("At {}: Ones {} | Zeros: {}", i, ones_count[i], zeros_count[i]);
        if ones_count[i] > zeros_count[i] {
            // most common value of index i is one, gamma[i] is 1
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        } else if ones_count[i] < zeros_count[i] {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        } else {
            panic!("Expect ones_count and zeros_count to differ in index {}, got {} for both", i, ones_count[i]);
        }
    }
    // println!("gamma rate: {} epsilon rate: {}", gamma_rate_string, epsilon_rate_string);
    let gamma_rate = i32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_string, 2).unwrap();
    let power_consumption = epsilon_rate * gamma_rate;
    return power_consumption;
}

fn day3_part2(data: &Vec<String>) -> i32 {
    // part 2: life support rating
    // filter to get only 1 valid number
    let valid_data: Vec<String> = data.clone();
    // final for O2 is keep more
    let final_value = filter_rating(&valid_data, 0, filter_keep_more);
    let oxygen_rating = i32::from_str_radix(&final_value, 2).unwrap();

    let valid_data: Vec<String> = data.clone();
    // final for CO2 is keep less
    let final_value = filter_rating(&valid_data, 0, filter_keep_less);
    let co2_rating = i32::from_str_radix(&final_value, 2).unwrap();

    let ls_rating = oxygen_rating * co2_rating;
    return ls_rating;
}

fn get_ones_and_zeros_count(data: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let string_length = data[0].len();
    let mut ones_count: Vec<i32> = vec![0; string_length];
    let mut zeros_count: Vec<i32> = vec![0; string_length];
    for binary_string in data {
        // iterate through each line and index it
        //println!("{}", binary_string);
        // assume first string has same length as all other strings
        for (index, digit) in binary_string.chars().enumerate() {
            //print!("{} {} ", index, digit);
            //print!("\n");
            if digit == '0' {
                zeros_count[index] += 1;
            } else if digit == '1' {
                ones_count[index] += 1;
            } else {
                panic!("Expected a 0 or 1 but got {} on string {} digit {}", digit, binary_string, index);
            }
        }
    }
    return (zeros_count, ones_count);
}

fn filter_rating(data: &Vec<String>, i: usize, filter_fun: fn(i32, i32, char) -> bool ) -> String {
    let (zeros_count, ones_count) = get_ones_and_zeros_count(&data);
    let mut valid_data: Vec<String> = vec![];
    // println!("Entering filter oxygen: index {}, 0c = {}, 1c = {}", i, zeros_count[i], ones_count[i]);
    for binary_string in data {
        // if ones count > zeros count, keep ones
        if filter_fun(ones_count[i], zeros_count[i], binary_string.chars().nth(i).unwrap()) {
            valid_data.push(binary_string.to_string());
        }
    }
    if valid_data.len() == 1 {
        return valid_data[0].to_string();
    }
    return filter_rating(&valid_data, i+1, filter_fun);
}

fn filter_keep_more(ones_count: i32, zeros_count: i32, character: char) -> bool {
    // use for O2 rating
    if (ones_count >= zeros_count) & (character == '1') {
        return true;
    } else if (zeros_count > ones_count) & (character == '0') {
        return true;
    } 
    return false;
}

fn filter_keep_less(ones_count: i32, zeros_count: i32, character: char) -> bool {
    // use for CO2 rating
    if (ones_count < zeros_count) & (character == '1') {
        return true;
    } else if (zeros_count <= ones_count) & (character == '0') {
        return true;
    }
    return false;
}

fn read_binary_file(path_string: &'static str) -> Vec<String> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<String> = vec![];
    for line in lines {
        // just return data as a vector of binary strings, we'll parse in the functions themselves
        data.push(line.to_string());
    }
    return data;
}

fn main() {
    println!("AOC2021 Day 3: imagine code standards and doing things rustically");
    // lessons learned:
    // read up on differences between String and &str and why you can't index in String

    let data: Vec<String> = read_binary_file("./input_sample.txt");
    let result = day3_part1(&data);
    let result2 = day3_part2(&data);
    println!("Sample: {} | Part 2: {}", result, result2);
    assert_eq!(result, 198);
    assert_eq!(result2, 230);

    let data: Vec<String> = read_binary_file("./input.txt");
    let result = day3_part1(&data);
    let result2 = day3_part2(&data);
    println!("Part 1: {} | Part 2: {}", result, result2);
    assert_eq!(result, 749376);
    assert_eq!(result2, 2372923);
}

