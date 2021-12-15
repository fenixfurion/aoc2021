use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::{LinkedList, HashMap};

#[derive(Clone, Debug)]
struct Polymer {
    template_string: String,
    chain: LinkedList<char>,
    rules: HashMap<String, char>,
    counts: HashMap<char, usize>,
}

impl Polymer {
    pub fn new(template_string: String, rules: HashMap<String, char>) -> Self {
        let mut chain = LinkedList::<char>::new();
        for char in template_string.chars() {
            chain.push_back(char);
        }
        let new_obj = Polymer { template_string: template_string, chain: chain, rules: rules, counts: HashMap::<char, usize>::new()};
        return new_obj
    }
    pub fn step(&mut self) {
        let mut new_chain = LinkedList::<char>::new();
        let mut iter = self.chain.iter();
        let mut next = iter.next().unwrap();
        let mut new_counts = HashMap::<char, usize>::new();
        loop {
            let curr = next;
            let next_iter = iter.next();
            if next_iter.is_none() {
                new_chain.push_back(curr.clone());
                let count = new_counts.entry(curr.clone()).or_insert(0);
                *count += 1;
                break;
            }
            next = next_iter.unwrap();
            // do insertion logic
            let pair = String::from_iter([curr, next]);
            //println!("Current pair: {}{}", curr, next);
            if self.rules.contains_key(&pair) {
                let insert_char = self.rules[&pair];
                //println!("Found rule {}{} -> {}", curr, next, insert_char);
                new_chain.push_back(curr.clone());
                let count = new_counts.entry(curr.clone()).or_insert(0);
                *count += 1;
                new_chain.push_back(insert_char);
                let count = new_counts.entry(insert_char).or_insert(0);
                *count += 1;
            } else {
                new_chain.push_back(curr.clone());
                let count = new_counts.entry(curr.clone()).or_insert(0);
                *count += 1;
            }
        }
        self.counts = new_counts;
        self.chain = new_chain;
    }
}

fn read_input_file(path_string: &'static str) -> Polymer {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let template_string = lines[0].to_string();
    let mut rules = HashMap::<String, char>::new();
    for line in lines {
        if line.contains("->") {
            // parse fold
            let data: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
            assert_eq!(data.len(), 2);
            assert_eq!(data[1].len(), 1);
            rules.insert(data[0].clone(), data[1].as_bytes()[0] as char);
        } 
    }
    let polymer: Polymer = Polymer::new(template_string, rules);
    return polymer;
}

fn get_counts(template_string: String, pair_freqs: &HashMap::<String, u64>) -> HashMap<char, u64> {
    let mut counts = HashMap::<char, u64>::new();
    for key in pair_freqs.keys() {
        for char in key.chars() {
            let count = counts.entry(char).or_insert(0);
            *count += pair_freqs[key];
        }
    }
    // add one to the first character before halving 
    let first_char = template_string.as_bytes()[0] as char;
    let count = counts.entry(first_char).or_insert(0);
    *count += 1;
    // subtract one from the last character before halving
    let last_char = template_string.as_bytes()[template_string.len()-1] as char;
    let count = counts.entry(last_char).or_insert(0);
    *count -= 1;
    // we've double counted so halve everything
    let keys = counts.keys().cloned().collect::<Vec<_>>();
    for key in keys {
        // assert_eq!(0, counts[&key] % 2);
        let count = counts.entry(key).or_insert(0);
        // println!("Char {} has {} occurrences", key, count);
        *count /= 2;
    }
    // subtract one from the first character after halving 
    let first_char = template_string.as_bytes()[0] as char;
    let count = counts.entry(first_char).or_insert(0);
    *count -= 1;
    // add one to the last character after halving
    let last_char = template_string.as_bytes()[template_string.len()-1] as char;
    let count = counts.entry(last_char).or_insert(0);
    *count += 1;
    return counts;
}

fn day14_part2(mut polymer: Polymer, steps: u64) -> u64 {
    let mut result = 0;
    let template_string = polymer.template_string.clone();
    // println!("{:?}", polymer);
    let mut pair_freqs = HashMap::<String, u64>::new();
    for i in 0..template_string.len()-1 {
        let pair = String::from_iter([template_string.as_bytes()[i] as char, template_string.as_bytes()[i+1] as char]);
        let count = pair_freqs.entry(pair).or_insert(0);
        *count += 1;
    }
    // println!("Pair freqs: {:?}", pair_freqs);
    let counts = get_counts(template_string.clone(), &pair_freqs);
    // println!("Initial counts: {:?}", counts);
    for step in 1..=steps {
        // println!("Pair freqs: {:?}", pair_freqs);
        let mut next_pair_freqs = HashMap::<String, u64>::new();
        for key in polymer.rules.keys() {
            if pair_freqs.contains_key(key) {
                let insert_char = polymer.rules[key];
                let lhs = key.as_bytes()[0] as char;
                let rhs = key.as_bytes()[1] as char;
                let lhs_new_pair = String::from_iter([lhs, insert_char]);
                let count = next_pair_freqs.entry(lhs_new_pair).or_insert(0);
                *count += pair_freqs[key];
                let rhs_new_pair = String::from_iter([insert_char, rhs]);
                let count = next_pair_freqs.entry(rhs_new_pair).or_insert(0);
                *count += pair_freqs[key];
            }
        }
        pair_freqs = next_pair_freqs;
        let counts = get_counts(template_string.clone(), &pair_freqs);
        // println!("Step: {}, Counts: {:?}", step, counts);
    }
    let counts = get_counts(template_string.clone(), &pair_freqs);
    // println!("Final counts: {:?}", counts);
    let mut max_count = 0 as u64;
    let mut min_count = u64::MAX;
    let mut total_count = 0;
    for key in counts.keys() {
        let count = counts[key];
        total_count += count;
        if count > max_count {
            max_count = count;
        }
        if count < min_count {
            min_count = count;
        }
    }
    println!("Total count of chars: {}", total_count);
    result = max_count-min_count;
    return result as u64;
}

fn day14_part1(mut polymer: Polymer, steps: u64) -> u64 {
    let mut result = 0;
    // println!("{:?}", polymer);
    for step in 1..=steps {
        polymer.step();
        // println!("Step {}: {:?}", step, polymer.chain);
        // println!("Step: {}, Len: {}, Counts: {:?}", step, polymer.chain.len(), polymer.counts);
    }
    let mut max_count = 0 as usize;
    let mut min_count = usize::MAX;
    for key in polymer.counts.keys() {
        let count = polymer.counts[key];
        if count > max_count {
            max_count = count;
        }
        if count < min_count {
            min_count = count;
        }
    }
    result = max_count - min_count;
    return result as u64;
}
fn main() {
    println!("AOC2021 Day 14: i cast polymerization");

    let polymer = read_input_file("./input_sample.txt");
    let result = day14_part1(polymer, 10);
    println!("Day 14 Sample 1: Result: {}", result);
    assert_eq!(result, 1588);

    let polymer = read_input_file("./input.txt");
    let result = day14_part1(polymer, 10);
    println!("Day 14 Puzzle 1: Result: {}", result);
    assert_eq!(result, 3048);

    let polymer = read_input_file("./input_sample.txt");
    let result = day14_part2(polymer, 40);
    println!("Day 14 Sample 2: Result: {}", result);
    assert_eq!(result, 2188189693529);

    let polymer = read_input_file("./input.txt");
    let result = day14_part2(polymer, 40);
    println!("Day 14 Sample 2: Result: {}", result);
}
