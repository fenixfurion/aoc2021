use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn read_bracket_file(path_string: &'static str) -> Vec<Vec<i32>> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<Vec<i32>> = vec![];
    for line in lines {
        let mut row: Vec<i32> = vec![];
        for character in line.chars() {
            row.push(character.to_digit(10).unwrap() as i32);
        }
        data.push(row);
    }
    return data;
}

fn day11_part1(data: &mut Vec<Vec<i32>>, days: i32, check_sync: bool) -> i64 {
    let mut total_flashes= 0;
    let height = data.len();
    let width = data[0].len();
    for day in 0..days {
        let mut flashed_list: Vec<Vec<bool>> = vec![vec![false; width]; height];
        // println!("Step {}", day);
        let mut pending_flashes: Vec<(usize,usize)> = vec![];
        for row in 0..data.len() {
            for col in 0..data[0].len() {
                data[row][col] += 1;
                if data[row][col] > 9 {
                    pending_flashes.push((row,col));
                }
            }
        }
        while pending_flashes.len() > 0 {
            let coord = pending_flashes.pop().unwrap();
            let row = coord.0;
            let col = coord.1;
            if flashed_list[row][col] == true {
                continue
            }
            flashed_list[row][col] = true;
            total_flashes += 1;
            // println!("Flash at {:?}", (coord));
            data[row][col] = 0;
            let adj_row_min = i32::max(row as i32-1,0) as usize;
            let adj_row_max = usize::min(row+1,height) as usize;
            let adj_col_min = i32::max(col as i32-1,0) as usize;
            let adj_col_max = usize::min(col+1,width) as usize;
            for adj_row in adj_row_min..=adj_row_max {
                for adj_col in adj_col_min..=adj_col_max {
                    if adj_col >= width || adj_row >= height {
                        continue;
                    } 
                    if flashed_list[adj_row][adj_col] == true {
                        continue
                    }
                    // add 1 to adjacent if not flashed
                    data[adj_row][adj_col] += 1;
                    if data[adj_row][adj_col] == 10 {
                        pending_flashes.push((adj_row,adj_col));
                    }
                }
            }
        }
        if check_sync {
            let mut all_flashed_today = true;
            for row in 0..height {
                for col in 0..width {
                    if flashed_list[row][col] == false {
                        all_flashed_today = false;
                    }
                }
            }
            if all_flashed_today {
                // println!("Everyone flashed on step {}", day+1);
                return (day+1) as i64;
            }
        }
    }
    return total_flashes;
}

fn main() {
    println!("AOC2021 Day 11: glowsquids? no, gloctopi");

    let mut data = read_bracket_file("./input_sample.txt");
    let result = day11_part1(&mut data, 100, false);
    println!("Day 11 Sample 1: Result: {}", result);

    let mut data = read_bracket_file("./input.txt");
    let result = day11_part1(&mut data, 100, false);
    println!("Day 11 Puzzle 1: Result: {}", result);

    let mut data = read_bracket_file("./input_sample.txt");
    let result = day11_part1(&mut data, 3333, true);
    println!("Day 11 Puzzle 2: Result: {}", result);

    let mut data = read_bracket_file("./input.txt");
    let result = day11_part1(&mut data, 3333, true);
    println!("Day 11 Puzzle 2: Result: {}", result);
    // let data = read_bracket_file("./input.txt");
    // let result = day11_part1(&data);
    // println!("Day 9 Puzzle 1: Result: {}", result);
}
