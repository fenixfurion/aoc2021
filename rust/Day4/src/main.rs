use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

#[derive(Clone)]
struct BingoBoard {
    width: usize,
    height: usize,
    board: Vec<Vec<i32>>,
    called: Vec<Vec<bool>>,
    score: i32,
    called_numbers: Vec<i32>,
}

impl BingoBoard {
    pub fn new(w: usize, h: usize) -> Self {
        BingoBoard { 
            width: w, 
            height: h,
            board: vec![vec![0; w]; h],
            called: vec![vec![false; w]; h],
            score: 0,
            called_numbers: vec![],
        }
    }
    fn calculate_core(&self) {
        self.score;
    }
    pub fn print_board(&self) {
        println!("Board @{:p}", self);
        for rindex in 0..self.height {
            for cindex in 0..self.width {
                let mut done = "";
                if self.called[rindex][cindex] == true {
                    done = "*";
                }
                print!("{}{}{}\t", done, self.board[rindex][cindex], done);
            }
            println!(" ");
        }
    }
    pub fn call_number(&mut self, number: i32) {
        self.called_numbers.push(number);
        for rindex in 0..self.height {
            for cindex in 0..self.width {
                if self.board[rindex][cindex] == number {
                    // println!("Row {} Col {} had value {}, activating", rindex, cindex, number);
                    self.called[rindex][cindex] = true;
                }
            }
        }
    }
    pub fn has_bingo(&self) -> bool {
        let mut bingo = false;
        // check all rows horizontally
        for rindex in 0..self.height {
            if self.called[rindex].iter().all(|&x| x == true) {
            bingo = true;
            }
        }
        for cindex in 0..self.width {
            let mut temp_bingo = true;
            for rindex in 0..self.width {
                if !self.called[rindex][cindex] {
                    temp_bingo = false;
                    break;
                }
            }
            if temp_bingo {
                bingo = true;
            }
        }
        // check all columns vertically
        return bingo;
    }
    pub fn get_score(&self) -> i32 {
        let mut score = 0;
        for rindex in 0..self.height {
            for cindex in 0..self.width {
                if !self.called[rindex][cindex] {
                    score += self.board[rindex][cindex];
                }
            }
        }
        return score*self.called_numbers.last().unwrap();
    }
}

fn read_board_file(path_string: &'static str) -> (Vec<i32>, Vec<BingoBoard>) {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    // parse the order of numbers
    let draw_order: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let width = 5;
    let height = 5;
    let mut bingo_cards = vec![];
    let mut new_board = true;
    let mut board_row_index = 0;
    let mut board = BingoBoard::new(width, height);
    for (index, line) in lines.iter().enumerate() {
        if index == 0 || line.is_empty() {
            // skip line
            continue
        }
        if new_board {
            // create a new board
            board = BingoBoard::new(width, height);
            new_board = false;
        }
        for (column, value) in line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).enumerate() {
            // println!("adding value {} to board at row {} column {}", value, board_row_index, column);
            board.board[board_row_index][column] = value;
        }
        board_row_index += 1;
        if board_row_index >= height {
            bingo_cards.push(board.clone());
            new_board = true;
            board_row_index = 0;
        }
    }
    return (draw_order, bingo_cards);
}

fn day4_part1(draw_order: &Vec<i32>, data: Vec<BingoBoard>) -> i32 {
    let mut boards = data;
    let mut game_over = false;
    let final_score = 0;
    for index in 0..draw_order.len() {
        // println!("CALL #{}: Value: {}", index, draw_order[index]);
        for b in 0..boards.len() {
            boards[b].call_number(draw_order[index]);
            if boards[b].has_bingo() {
                // boards[b].print_board();
                let final_score = boards[b].get_score();
                // println!("Board {} has bingo! Score: {}", b, final_score);
                game_over = true;
                return final_score;
                break;
            }
        }
        if game_over {
            break;
        }
    }
    // test suite
    /*
    for number in draw_order {
        println!("{}", number);
    }
    for i in 0..boards.len() {
        boards[i].print_board();
        boards[i].call_number(21);
        boards[i].call_number(0);
        boards[i].call_number(8);
        boards[i].call_number(11);
        if boards[i].has_bingo() {
            println!("Board has bingo");
        } else {
            println!("Board doesn't have bingo yet");
        }
        boards[i].call_number(16);
        boards[i].print_board();
        if boards[i].has_bingo() {
            println!("Board has bingo");
        }
    }
    */
    return 0;
}

fn day4_part2(draw_order: &Vec<i32>, data: Vec<BingoBoard>) -> i32 {
    let mut boards = data;
    let mut done_boards = 0;
    let mut last_score = 0;
    for index in 0..draw_order.len() {
        // println!("CALL #{}: Value: {}", index, draw_order[index]);
        for b in 0..boards.len() {
            if boards[b].has_bingo() {
                continue;
            }
            boards[b].call_number(draw_order[index]);
            if boards[b].has_bingo() {
                // boards[b].print_board();
                // println!("Board {} has bingo! Score: {}", b, boards[b].get_score());
                last_score = boards[b].get_score();
                done_boards += 1;
            }
        }
        if done_boards == boards.len() {
            // println!("All boards have bingo now!");
            break;
        }
    }
    return last_score;
}

fn main() {
    println!("AOC2021 Day 4: structs on structs in structs");
    // lessons learned:
    // read up on differences between String and &str and why you can't index in String

    let (draw_order, data) = read_board_file("./input_sample.txt");
    let result = day4_part1(&draw_order, data);
    println!("Day 4 Sample 1: First Board Score: {}", result);
    let (draw_order, data) = read_board_file("./input.txt");
    let result = day4_part1(&draw_order, data);
    println!("Day 4 Puzzle 1: First Board Score: {}", result);

    let (draw_order, data) = read_board_file("./input_sample.txt");
    let result = day4_part2(&draw_order, data);
    println!("Day 4 Sample 1: Last Board Score: {}", result);
    let (draw_order, data) = read_board_file("./input.txt");
    let result = day4_part2(&draw_order, data);
    println!("Day 4 Sample 1: Last Board Score: {}", result);
}

