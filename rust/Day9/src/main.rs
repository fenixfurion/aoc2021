use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn read_map_file(path_string: &'static str) -> Vec<Vec<u32>> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<Vec<u32>> = vec![];
    for line in lines {
        let mut row: Vec<u32> = vec![];
        for character in line.chars() {
            row.push(character.to_digit(10).unwrap());
        }
        data.push(row);
    }
    return data;
}

fn is_lowpoint(data: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    let mut lowpoint = true;
    let value = data[row][col];
    if row != 0 {
        if data[row-1][col] <= value {
            lowpoint = false;
        }
    }
    if row != data.len()-1 {
        if data[row+1][col] <= value {
            lowpoint = false;
        }
    }
    if col != 0 {
        if data[row][col-1] <= value {
            lowpoint = false;
        }
    }
    if col != data[0].len()-1 {
        if data[row][col+1] <= value {
            lowpoint = false;
        }
    }
    return lowpoint
}

fn day9_part1(data: &Vec<Vec<u32>>) -> u32 {
    let mut risk = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            print!("{}", data[row][col]);
            if is_lowpoint(data, row, col) {
                risk += 1 + data[row][col];
            }
        }
        println!("");
    }
    return risk;
}

fn day9_part2(data: &Vec<Vec<u32>>) -> u32 {
    let mut prod = 1;
    let mut basins: Vec<HashSet<(usize, usize)>> = vec![];
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            let new_basin = get_basin_from_coord(data, row, col);
            if new_basin.len() > 0 {
                basins.push(new_basin);
            }
        }
    }
    let mut basin_lengths: Vec<usize> = vec![];
    for i in 0..basins.len() {
        // println!("{}", basins[i].len());
        basin_lengths.push(basins[i].len());
    }
    basin_lengths.sort();
    for i in basin_lengths.len()-3..basin_lengths.len() {
        prod *= basin_lengths[i] as u32;
    }
    return prod
}

fn get_basin_from_coord(data: &Vec<Vec<u32>>, row: usize, col: usize) -> HashSet<(usize, usize)> {
    let mut basin = HashSet::<(usize, usize)>::new();
    if data[row][col] >= 9 {
        return basin
    }
    let mut last_size = 0;
    let mut current_size = 1;
    basin.insert((row,col));
    while last_size < current_size {
        last_size = current_size;
        let mut coords_to_add = HashSet::<(usize, usize)>::new();
        for coords in basin.iter() {
            coords_to_add = coords_to_add.union(&get_higher_cells(data, coords.0, coords.1)).copied().collect();
        }
        basin = basin.union(&coords_to_add).copied().collect();
        current_size = basin.len();
    }
    return basin
}

fn get_higher_cells(data: &Vec<Vec<u32>>, row: usize, col: usize) -> HashSet<(usize, usize)> {
    let mut higher_cells = HashSet::<(usize,usize)>::new();
    let value = data[row][col];
    if row != 0 {
        if data[row-1][col] > value && data[row-1][col] < 9 {
            higher_cells.insert((row-1,col));
        }
    }
    if row != data.len()-1 {
        if data[row+1][col] > value && data[row+1][col] < 9 {
            higher_cells.insert((row+1,col));
        }
    }
    if col != 0 {
        if data[row][col-1] > value && data[row][col-1] < 9 {
            higher_cells.insert((row,col-1));
        }
    }
    if col != data[0].len()-1 {
        if data[row][col+1] > value && data[row][col+1] < 9 {
            higher_cells.insert((row,col+1));
        }
    }
    return higher_cells
}

fn main() {
    println!("AOC2021 Day 9: maps (but not hashmaps)");

    let data = read_map_file("./input_sample.txt");
    let result = day9_part1(&data);
    println!("Day 9 Sample 1: Result: {}", result);

    let data = read_map_file("./input_sample.txt");
    let result = day9_part2(&data);
    println!("Day 9 Sample 2: Result: {}", result);

    let data = read_map_file("./input.txt");
    let result = day9_part1(&data);
    println!("Day 9 Puzzle 1: Result: {}", result);

    let data = read_map_file("./input.txt");
    let result = day9_part2(&data);
    println!("Day 9 Puzzle 2: Result: {}", result);
}
