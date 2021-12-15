use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn read_input_file(path_string: &'static str) -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut folds: Vec<(char, u32)> = vec![];
    let mut dots = HashSet::<(u32,u32)>::new();
    for line in lines {
        if line.contains("fold along") {
            // parse fold
            let data: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
            assert_eq!(data.len(), 3);
            let fold_info: Vec<&str> = data[2].split("=").collect();
            assert_eq!(fold_info.len(),2);
            folds.push((fold_info[0].as_bytes()[0] as char, fold_info[1].parse::<u32>().unwrap()));
        } else {
            let data_split: Vec<&str> = line.split(",").collect();
            if data_split.len() == 2 {
                let data: Vec<u32> = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
                dots.insert((data[0],data[1]));
            }
        }
    }
    return (dots, folds);
}

fn day13_part1(dots: HashSet<(u32, u32)>, folds: Vec<(char, u32)>) -> HashSet<(u32, u32)> {
    let result;
    // println!("Dots ({} visible): {:?}", dots.len(), dots);
    // println!("Folds: {:?}", folds);
    let mut current_dots = dots.clone();
    for fold in folds {
        if fold.0 == 'x' {
            let x_fold = fold.1;
            // do x stuff (vertical line)
            // println!("Folding vertically at {}={}", fold.0, fold.1);
            let mut next_dots: HashSet<(u32, u32)> = HashSet::<(u32,u32)>::new();
            for coord in current_dots.iter() {
                let (x, y) = coord.clone();
                if x > x_fold {
                    let x_new = x_fold + x_fold - x;
                    next_dots.insert((x_new,y));
                } else {
                    next_dots.insert((x,y));
                }
            }
            current_dots = next_dots;
        } else if fold.0 == 'y' {
            let y_fold = fold.1;
            // do y stuff (horizontal line)
            // println!("Folding horizontally at {}={}", fold.0, fold.1);
            /* Folding logic *\
            Fold bottom half up:
            Keep X value
            Assume no dots exist in Y = y_fold
            New Y-coordinate if Y > y_fold: y_new = y_fold+yfold-old_y
            */
            let mut next_dots: HashSet<(u32, u32)> = HashSet::<(u32,u32)>::new();
            for coord in current_dots.iter() {
                let (x, y) = coord.clone();
                if y > y_fold {
                    let y_new = y_fold + y_fold - y;
                    next_dots.insert((x,y_new));
                } else {
                    next_dots.insert((x,y));
                }
            }
            current_dots = next_dots;
        } else {
            panic!("Expected 'x' or 'y', got {} instead for fold type", fold.0);
        }
        // println!("Current Dots after fold: ({} visible) {:?}", current_dots.len(), current_dots);
    }
    result = current_dots;
    return result;
}

fn day13_part2(dots: HashSet<(u32, u32)> ) {
    let mut x_max = 0;
    let mut y_max = 0;
    for coord in dots.iter() {
        if coord.0 > x_max {
            x_max = coord.0;
        }
        if coord.1 > y_max {
            y_max = coord.1;
        }
    }
    for y in 0..=y_max {
        for x in 0..=x_max {
            if dots.contains(&(x,y)) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    println!("AOC2021 Day 13: ????");

    let (dots, folds) = read_input_file("./input_sample.txt");
    let result = day13_part1(dots, folds);
    println!("Day 13 Sample 1: Result: {}", result.len());
    day13_part2(result);

    let (dots, folds) = read_input_file("./input.txt");
    let result = day13_part1(dots, folds);
    println!("Day 13 Puzzle 1: Result: {}", result.len());
    day13_part2(result);
}
