use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Clone)]
struct Aquarium {
    timers: HashMap::<u128, u128>,
    timer_max: u128,
}

impl Aquarium {
    pub fn new(inital_pop: &Vec<u128>) -> Self {
        let mut new_obj: Aquarium = Aquarium {
            timers: HashMap::<u128, u128>::new(),
            timer_max: 8,
        };
        for time in 0u128..=new_obj.timer_max {
            new_obj.timers.insert(time, 0);
        }
        for i in 0..inital_pop.len() {
            let count = new_obj.timers.entry(inital_pop[i]).or_insert(0);
            *count += 1;
        }
        return new_obj;
    }
    pub fn step_days(&mut self, days: u128) {
        for _i in 0..days {
            // println!("Day {}", i);
            self.step_day();
        }
    }
    pub fn step_day(&mut self) {
        let mut next_timers = HashMap::<u128, u128>::new();
        let mut reset_fish: u128 = 0;
        // println!("Stepping day");
        for time in 0u128..=8u128 {
            if time == 0u128 {
                reset_fish = *self.timers.get(&time).unwrap();
                // println!("There are {} fish about to go to 6 and create 8", reset_fish);
            } else {
                next_timers.insert(time-1, *self.timers.get(&time).unwrap());
            }
        }
        // println!("Reset fish: {}", reset_fish);
        *next_timers.entry(6).or_insert(0) += reset_fish;
        *next_timers.entry(8).or_insert(0) += reset_fish;
        self.timers = next_timers;
        // for k in 0..=self.timer_max {
        //     println!("Next_Timer {}: {}", k, self.timers.get(&k).unwrap());
        // }
    }
    pub fn print(&self) {
        println!("Fish: ");
        for k in 0..=self.timer_max {
            println!("Next_Timer {}: {}", k, self.timers.get(&k).unwrap());
        }
        println!("Total Fish: {}", self.get_fish());
    }
    pub fn get_fish(&self) -> u128 {
        let mut sum = 0;
        for i in 0..=self.timer_max {
            sum += *self.timers.get(&i).unwrap();           
        }
        return sum;
    }
}

fn day6_part1(data: &Vec<u128>, days: u128) -> u128 {
    let mut fish: Aquarium = Aquarium::new(data);
    // fish.print();
    fish.step_days(days);
    // fish.print();
    return fish.get_fish();
}

fn read_line_file(path_string: &'static str) -> Vec<u128> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let data: Vec<u128> = lines[0].split(",").map(|x| x.parse::<u128>().unwrap()).collect();
    return data;
}

fn main() {
    println!("AOC2021 Day 6: please god");

    let data = read_line_file("./input_sample.txt");
    let result = day6_part1(&data, 80);
    println!("Day 6 Sample 1: Result: {}", result);
    assert_eq!(result, 5934);

    let data = read_line_file("./input.txt");
    let result = day6_part1(&data, 80);
    println!("Day 6 Sample 1: Result: {}", result);
    assert_eq!(result, 393019);

    let data = read_line_file("./input_sample.txt");
    let result = day6_part1(&data, 256);
    println!("Day 6 Sample 1: Result: {}", result);
    assert_eq!(result, 26984457539);

    let data = read_line_file("./input.txt");
    let result = day6_part1(&data, 256);
    println!("Day 6 Sample 1: Result: {}", result);
    assert_eq!(result, 1757714216975);

    // let result = day6_part1(&data, 5120);
    // println!("Day 6 Sample 1: Result: {}", result);
}
