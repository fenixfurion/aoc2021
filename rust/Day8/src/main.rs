use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Clone)]
struct SSDisplay {
    signal_patterns: Vec<String>,
    output_values: Vec<String>,
    mapping: Vec<HashSet<char>>,
    pattern_map: Vec<String>
}
/*
Seven segment display indices:
 0
1 2
 3
4 5 
 6
*/

impl SSDisplay {
    pub fn new(signal_patterns: Vec<&str>, output_values: Vec<&str>) -> Self {
        let mut new_obj: SSDisplay = SSDisplay {
            signal_patterns: vec![],
            output_values: vec![],
            mapping: vec![HashSet::<char>::new(); 7],
            pattern_map: vec![String::new(); 10],
        };
        for elem in signal_patterns {
            new_obj.signal_patterns.push(elem.to_string());
        }
        for elem in output_values {
            new_obj.output_values.push(elem.to_string());
        }

        for i in 0..7 {
            for character in 'a'..='g' {
                new_obj.mapping[i].insert(character);
            }
        }
        new_obj.determine_mapping();
        return new_obj;
    }
    pub fn print(&self) {
        println!("SSDisplay signal patterns:");
        for elem in &self.signal_patterns {
            print!("{} ", elem);
        }
        println!("");
        println!("SSDisplay output values:");
        for elem in &self.output_values {
            print!("{} ", elem);
        }
        println!("");
    }
    pub fn count_1_4_7_8(&self) -> u32 {
        let mut sum = 0;
        // for elem in &self.signal_patterns {
        //     if elem.len() == 2 || elem.len() == 3 || elem.len() == 6 || elem.len() == 4 {
        //         sum += 1;
        //     }
        // }
        for elem in &self.output_values {
            if elem.len() == 2 || elem.len() == 3 || elem.len() == 7 || elem.len() == 4 {
                sum += 1;
                // println!("{}", elem);
            }
        }
        return sum;
    }
    pub fn is_solved(&self) -> bool {
        let mut is_solved = true;
        for i in 0..7 {
            // println!("{}", self.mapping[i].len());
            if self.mapping[i].len() != 1 {
                is_solved = false;
            }
        }
        // println!("Solved!");
        return is_solved
    }
    pub fn determine_mapping(&mut self) {
        // determine mapping - start with the unique ones and place those characters in 
        // the candidate mapping slots
        // based on get_cand_indices()
        let mut break_index = 0;
        while !self.is_solved() {
            // println!("Solving again...");
            // for dig_index in 0..7 {
            //     print!("Digit {}: ", dig_index);
            //     for key in &self.mapping[dig_index as usize] {
            //         print!("{:?} ", key);
            //     }
            //     println!("");
            // }
            for i in 0..self.signal_patterns.len() {
                let elem = &self.signal_patterns[i];
                let pattern = pattern_to_set(elem.to_string());
                let mut sure_digit = false;
                let mut digit = 0;
                // easy cases where digits are already gotten
                if elem.len() == 2 {
                    // 1 (pos 2 or 5)
                    sure_digit = true;
                    digit = 1;
                    self.pattern_map[1] = elem.to_string();
                } else if elem.len() == 3 {
                    // 7 (pos 0, 2, and 5)
                    sure_digit = true;
                    digit = 7;
                    self.pattern_map[7] = elem.to_string();
                } else if elem.len() == 4 {
                    // 4 (pos 1, 2, 3, and 5)
                    // ignore len 7 for now since it adds everything as a candidate
                    sure_digit = true;
                    digit = 4;
                    self.pattern_map[4] = elem.to_string();
                }
                // other algorithm steps
                // 6
                if elem.len() == 6 && self.mapping[2].len() == 2 {
                    // 6 is either 0, 6, or 9
                    // in 6, digit 2 is missing which can tell us which character
                    // corresponds to digit 2 and 5
                    let result_set = intersect_candidate_sets(&pattern, &self.mapping[2]);
                    if result_set.len() == 1 {
                        // if only 1 left, the remaining one is digit 5 and the missing one is digit 2
                        self.mapping[5] = result_set;
                        self.mapping[2] = remove_candidates(&self.mapping[2], &self.mapping[5]);
                        // this has to be 6
                        sure_digit = true;
                        digit = 6;
                        self.pattern_map[6] = elem.to_string();
                    }
                }
                // we anchored digits 2 and 5 already, solve via a 2, 3, or 5
                if elem.len() == 5 && self.mapping[2].len() == 1 {
                    let check_3 = intersect_candidate_sets(&pattern, &pattern_to_set(self.pattern_map[1].to_string()));
                    if check_3.len() == 2 {
                        // this HAS to be 3, digit 3 is solved now
                        let result_set = intersect_candidate_sets(&self.mapping[3], &pattern);
                        sure_digit = true;
                        digit = 3;
                        self.pattern_map[3] = elem.to_string();
                        // println!("Found 3 in pattern {}", elem);
                        // self.mapping[3] = intersect_candidate_sets(&self.mapping[3], &pattern);
                    }
                }
                if sure_digit {
                    for index in get_cand_indices(digit) {
                        self.mapping[index as usize] = intersect_candidate_sets(&self.mapping[index as usize], &pattern);
                    }
                    for index in get_inactive_indexes(digit) {
                        self.mapping[index as usize] = remove_candidates(&self.mapping[index as usize], &pattern);
                    }
                }
            }
            break_index += 1;
            if break_index == 10 {
                panic!("Looped too many times trying to solve for this pattern.");
                break;
            }
        }
        // for dig_index in 0..7 {
        //     print!("Digit {}: ", dig_index);
        //     for key in &self.mapping[dig_index as usize] {
        //         print!("{:?} ", key);
        //     }
        //     println!("");
        // }
    }
    pub fn interpret_digit(&self, pattern: String) -> i32 {
        for i in 0..10 {
            let mut digit_pattern: HashSet<char> = HashSet::<char>::new();
            for index in get_cand_indices(i) {
                digit_pattern = digit_pattern.union(&self.mapping[index as usize]).copied().collect();
                if pattern_to_set(pattern.to_string()) == digit_pattern {
                    return i as i32;
                }
            }
        }
        return 0;
    }
    pub fn get_output_value(&self) -> i32 {
        let mut sum = 0;
        let length: i32 = self.output_values.len() as i32;
        for i in 0..length {
           sum += i32::pow(10,(length-i-1) as u32)*self.interpret_digit(self.output_values[i as usize].to_string());
        }
        return sum;
    }
}
pub fn intersect_candidate_sets(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    return a.intersection(&b).copied().collect();
}

pub fn remove_candidates(start_set: &HashSet<char>, candidates: &HashSet<char>) -> HashSet<char> {
    return start_set.difference(&candidates).copied().collect();
}

pub fn get_inactive_indexes(digit: u32) -> Vec<u32> {
    let mut indices = vec![];
    let active_indices = get_cand_indices(digit);
    for index in 0..7 {
        if !active_indices.iter().any(|&i| i==index) {
            indices.push(index);
        }
    }
    return indices;
}

fn get_cand_indices(digit: u32) -> Vec<u32> {
    let indices;
    match digit {
        0 => indices = [0, 1, 2, 4, 5, 6].to_vec(),
        1 => indices = [2, 5].to_vec(),
        2 => indices = [0, 2, 3, 4, 6].to_vec(),
        3 => indices = [0, 2, 3, 5, 6].to_vec(),
        4 => indices = [1, 2, 3, 5].to_vec(),
        5 => indices = [0, 1, 3, 5, 6].to_vec(),
        6 => indices = [0, 1, 3, 4, 5, 6].to_vec(),
        7 => indices = [0, 2, 5].to_vec(),
        8 => indices = [0, 1, 2, 3, 4, 5, 6].to_vec(),
        9 => indices = [0, 1, 2, 3, 5, 6].to_vec(),
        _ => panic!("Expected a digit from 0..=9, got {} instead", digit),
    }
    return indices;
}

fn pattern_to_set(pattern: String) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::<char>::new();
    for character in pattern.chars() {
        set.insert(character);
    }
    return set
}

fn day8_part1(data: Vec<SSDisplay>) -> u32 {
    let mut sum = 0;
    let mut displays = data;
    for i in 0..displays.len() {
        // &data[i].print();
        // &displays[i].print();
        sum += &displays[i].count_1_4_7_8();
    }
    return sum;
}

fn day8_part2(data: Vec<SSDisplay>) -> u32 {
    let mut oval_sum = 0;
    let mut displays = data;
    for i in 0..displays.len() {
        // &data[i].print();
        // &displays[i].print();
        // sum += &displays[i].count_1_4_7_8();
        // println!("Output value: {}", );
        oval_sum += &displays[i].get_output_value();
    }
    return oval_sum as u32;
}

fn read_segment_file(path_string: &'static str) -> Vec<SSDisplay> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<SSDisplay> = vec![];
    for line in lines {
        let split_line: Vec<&str> = line.split("|").collect();
        let signal_pattern: Vec<&str> = split_line[0].trim().split(" ").collect();
        let output_values: Vec<&str> = split_line[1].trim().split(" ").collect();
        let temp_ssdisplay = SSDisplay::new(signal_pattern, output_values);
        data.push(temp_ssdisplay);
    }
    return data;
}

fn main() {
    println!("AOC2021 Day 6: please god");

    let data = read_segment_file("./input_sample.txt");
    let result = day8_part1(data);
    println!("Day 8 Sample 1 P1: Result: {}", result);
    let data = read_segment_file("./input_sample.txt");
    let result = day8_part2(data);
    println!("Day 8 Sample 1 P2: Result: {}", result);

    let data = read_segment_file("./input_sample_2.txt");
    let result = day8_part1(data);
    println!("Day 8 Sample 2 P1: Result: {}", result);
    let data = read_segment_file("./input_sample_2.txt");
    let result = day8_part2(data);
    println!("Day 8 Sample 2 P2: Result: {}", result);

    let data = read_segment_file("./input.txt");
    let result = day8_part1(data);
    println!("Day 8 Puzzle P1: Result: {}", result);
    let data = read_segment_file("./input.txt");
    let result = day8_part2(data);
    println!("Day 8 Puzzle P2: Result: {}", result);
}

