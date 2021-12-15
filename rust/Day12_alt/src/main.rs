use std::fs::File;
use std::iter::FromIterator;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Passage {
    nodes: HashMap<String, Cave>
}

impl Passage {
    pub fn new() -> Self {
        let start_cave = Cave::new("start".to_string());
        let end_cave = Cave::new("end".to_string());
        let mut nodes: HashMap<String, Cave> = HashMap::<String, Cave>::new();
        nodes.insert("start".to_string(), start_cave);
        nodes.insert("end".to_string(), end_cave);
        let new_obj = Passage {nodes: nodes};
        return new_obj
    }
    pub fn create_connection(&mut self, lhs: String, rhs: String) {
        let node = self.nodes.entry(lhs.to_string()).or_insert(Cave::new(lhs.to_string()));
        node.connections.push(rhs.to_string());
        let node = self.nodes.entry(rhs.to_string()).or_insert(Cave::new(rhs.to_string()));
        node.connections.push(lhs.to_string());
    }
    // pub fn traverse_double(&self, current_path: Vec<String>) -> Vec<Vec<String>> {
    //     let mut small_caves: Vec<String> = vec![];
    //     let mut return_paths: Vec<Vec<String>> = vec![];
    //     for key in self.nodes.keys() {
    //         if key == "start" || key == "end" {
    //             continue;
    //         } else if self.nodes[key].is_big == true {
    //             continue;
    //         } else {
    //             small_caves.push(key.to_string());
    //         }
    //     }
    //     small_caves.sort();
    //     for i in 0..small_caves.len() {
    //         let traverse_paths = self.traverse(current_path.clone(), small_caves[i].clone());
    //     }
    //     return return_paths
    // }
    pub fn traverse(&self, current_path: Vec<String>, double_allowed: bool) -> i32 {
        // base case
        let mut num_paths = 0;
        let mut return_paths: Vec<Vec<String>> = vec![];
        if current_path[current_path.len()-1] == "end" {
            // println!("Finished path: {:?}", current_path);
            return_paths.push(current_path);
            return 1;
        } else {
            let candidates = self.get_candidates(&current_path, double_allowed);
            // println!("Path: {:?}", current_path);
            // println!("Candidates: {:?}", candidates);
            for i in 0..candidates.len() {
                let mut candidate_path = current_path.clone();
                candidate_path.push(candidates[i].to_string());
                num_paths += self.traverse(candidate_path, double_allowed);
            }
        }
        return num_paths
    }
    pub fn get_candidates(&self, current_path: &Vec<String>, double_allowed: bool) -> Vec<String> {
        let current_cave = &self.nodes[&current_path[current_path.len()-1]];
        let mut candidates = current_cave.connections.clone();
        candidates.sort();
        let mut qualified_candidates: Vec<String> = vec![];
        for i in 0..candidates.len() {
            if candidates[i] == "start" {
                continue
            }
            // push if big
            if self.nodes[&candidates[i]].is_big {
                qualified_candidates.push(candidates[i].clone());
                continue
            }
            if double_allowed {
                if self.path_has_double_smalls(&current_path) {
                    if current_path.contains(&candidates[i]) {
                        continue
                    }
                }
            } else {
                if current_path.contains(&candidates[i]) {
                    continue
                }
            }
            qualified_candidates.push(candidates[i].clone());
        }
        return qualified_candidates;
    }
    pub fn path_has_double_smalls(&self, current_path: &Vec<String>) -> bool {
        let mut has_doubles: bool = false;
        for i in 0..current_path.len() {
            if !&self.nodes[&current_path[i]].is_big {
                if current_path.iter().filter(|&n| *n == current_path[i]).count() == 2 {
                    has_doubles = true;
                    // println!("Path has doubles: {:?}", current_path);
                }
            }
        }
        return has_doubles;
    }
}

#[derive(Clone)]
struct Cave {
    name: String,
    connections: Vec<String>,
    is_big: bool
}

impl Cave {
    pub fn new(name: String) -> Self {
        let mut new_obj: Cave = Cave { name: name, connections: vec![], is_big: false };
        if new_obj.name.to_ascii_uppercase() == new_obj.name {
            new_obj.is_big = true;
        }
        return new_obj
    }
}

fn read_cave_file(path_string: &'static str) -> Passage {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Passage = Passage::new();
    for line in lines {
        let connections = line.split('-').collect::<Vec<&str>>();
        data.create_connection(connections[0].to_string(),connections[1].to_string());
    }
    return data;
}

fn day12_part1(data: Passage) -> i32 {
    // for key in data.nodes.keys() {
    //     println!("Node {}, connections: {:?}", data.nodes[key].name, data.nodes[key].connections);
    // }
    let base_path: Vec<String> = vec!["start".to_string()];
    let num_paths = data.traverse(base_path, false);
    return num_paths;
}

fn day12_part2(data: Passage) -> i32 {
    // for key in data.nodes.keys() {
    //     println!("Node {}, connections: {:?}", data.nodes[key].name, data.nodes[key].connections);
    // }
    let base_path: Vec<String> = vec!["start".to_string()];
    let num_paths = data.traverse(base_path, true);
    return num_paths;
}

fn main() {
    println!("AOC2021 Day 12: traverse the world");

    let mut data = read_cave_file("./input_basic.txt");
    let result = day12_part1(data);
    println!("Day 12 Basic  1: Result: {}", result);
    assert_eq!(result, 10);

    let mut data = read_cave_file("./input_sample.txt");
    let result = day12_part1(data);
    println!("Day 12 Sample 1: Result: {}", result);
    assert_eq!(result, 19);

    let mut data = read_cave_file("./input_larger.txt");
    let result = day12_part1(data);
    println!("Day 12 Larger 1: Result: {}", result);
    assert_eq!(result, 226);

    let mut data = read_cave_file("./input.txt");
    let result = day12_part1(data);
    println!("Day 12 Puzzle 1: Result: {}", result);
    assert_eq!(result, 4241);

    let mut data = read_cave_file("./input_basic.txt");
    let result = day12_part2(data);
    println!("Day 12 Basic  2: Result: {}", result);
    assert_eq!(result, 36);

    let mut data = read_cave_file("./input_sample.txt");
    let result = day12_part2(data);
    println!("Day 12 Sample 2: Result: {}", result);
    assert_eq!(result, 103);

    let mut data = read_cave_file("./input_larger.txt");
    let result = day12_part2(data);
    println!("Day 12 Larger 2: Result: {}", result);
    assert_eq!(result, 3509);

    let mut data = read_cave_file("./input.txt");
    let result = day12_part2(data);
    println!("Day 12 Puzzle 2: Result: {}", result);
}
