use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn read_bracket_file(path_string: &'static str) -> Vec<Vec<char>> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<Vec<char>> = vec![];
    for line in lines {
        let mut row: Vec<char> = vec![];
        for character in line.chars() {
            row.push(character);
        }
        data.push(row);
    }
    return data;
}

fn day9_part1(data: &Vec<Vec<char>>) -> i64 {
    let mut score = 0;
    for line in data {
        // println!("{:?}", line);
        let (status, new_score) = parse_line(line);
        if status == 2 {
            // corrupt
            score += new_score;
        }
    }
    return score;
}


fn day9_part2(data: &Vec<Vec<char>>) -> i64 {
    let mut scores: Vec<i64> = vec![];
    for line in data {
        // println!("{:?}", line);
        let (status, new_score) = parse_line(line);
        if status == 1 {
            // corrupt
            scores.push(new_score);
        }
    }
    scores.sort();
    // there are always an odd number of incomplete lines
    assert_eq!(1, scores.len()%2);
    let score = scores[scores.len()/2];
    return score;
}

// return status, score
// 0 = good
// 1 = incomplete
// 2 = corrupt
fn parse_line(data: &Vec<char>) -> (i32, i64) {
    let mut status = 0;
    let mut score = 0;
    let mut parse_stack: Vec<char> = vec![];
    let mut expected_token = ' ';
    for index in 0..data.len() {
        let token = data[index];
        let mut expecting_token = false;
        if token == '{' || token == '(' || token == '[' || token == '<' {
            parse_stack.push(token);
            // println!("Parse stack: {:?}", parse_stack);
        } else if token == '}' {
            expecting_token = true;
            expected_token = '{';
        } else if token == ')' {
            expecting_token = true;
            expected_token = '(';
        } else if token == ']' {
            expecting_token = true;
            expected_token = '[';
        } else if token == '>' {
            expecting_token = true;
            expected_token = '<';
        }
        if expecting_token {
            let popped_token = parse_stack.pop().unwrap();
            // println!("Popped {}, expecting {}", popped_token, expected_token);
            if popped_token != expected_token {
                status = 2;
                break;
            }
            // otherwise, we're good
        }
    }
    if status == 2 {
        match expected_token {
            '(' => score = 3,
            '[' => score = 57,
            '{' => score = 1197,
            '<' => score = 25137,
            _   => panic!("Tried to match with unexpected token {}", expected_token),
        };
        // println!("  Line is corrupt; score = {}", score);
    } else {
        // check if incomplete
        if parse_stack.len() != 0 {
            // println!("  Line is incomplete.");
            // println!("  Parse stack: {:?}", parse_stack);
            status = 1;
            // now autocomplete the line
            for _ in 0..parse_stack.len() {
                let popped_token = parse_stack.pop().unwrap();
                // println!("  Popped {}", popped_token);
                match popped_token {
                    '(' => score = (score * 5)+1,
                    '[' => score = (score * 5)+2,
                    '{' => score = (score * 5)+3,
                    '<' => score = (score * 5)+4,
                    _ => panic!("Tried to autocomplete unexpected token {}", popped_token),
                }
                // println!("Autocomplete score: {}", score);
            }
        }
    }
    return (status, score)
}

fn main() {
    println!("AOC2021 Day 10: i never took a compilers course");

    let data = read_bracket_file("./input_sample.txt");
    let result = day9_part1(&data);
    println!("Day 9 Sample 1: Result: {}", result);

    let data = read_bracket_file("./input.txt");
    let result = day9_part1(&data);
    println!("Day 9 Puzzle 1: Result: {}", result);

    let data = read_bracket_file("./input_sample.txt");
    let result = day9_part2(&data);
    println!("Day 9 Sample 2: Result: {}", result);

    let data = read_bracket_file("./input.txt");
    let result = day9_part2(&data);
    println!("Day 9 Puzzle 2: Result: {}", result);
}
