use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::{HashMap};
use petgraph::algo::{self};
use petgraph::graph::{DiGraph, NodeIndex, EdgeIndex};

fn read_input_file(path_string: &'static str) -> Vec<Vec<u32>> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut data: Vec<Vec<u32>> = vec![];
    for line in lines {
        let mut row: Vec<u32> = vec![];
        for character in line.chars() {
            row.push(character.to_digit(10).unwrap() as u32);
        }
        data.push(row);
    }
    return data;
}

fn create_risk_graph(data: Vec<Vec<u32>>) -> (DiGraph<(usize, usize), u32>, HashMap<(usize, usize), NodeIndex>) {
    let mut graph = DiGraph::<(usize, usize), u32>::new();
    let height = data.len();
    let width = data[0].len();
    let mut nodeMap =  HashMap::<(usize, usize), NodeIndex>::new();
    // generate all nodes
    for row in 0..height {
        for col in 0..width {
            nodeMap.insert((row,col), graph.add_node((row, col)));
        }
    }
    // now add edges
    for row in 0..height {
        for col in 0..width {
            if row != 0 {
                graph.add_edge(nodeMap[&(row-1, col)], nodeMap[&(row, col)], data[row][col]);
            }
            if row+1 < height {
                graph.add_edge(nodeMap[&(row+1, col)], nodeMap[&(row, col)], data[row][col]);
            }
            if col != 0 {
                graph.add_edge(nodeMap[&(row, col-1)], nodeMap[&(row, col)], data[row][col]);
            }
            if col+1 < width {
                graph.add_edge(nodeMap[&(row, col+1)], nodeMap[&(row, col)], data[row][col]);
            }
        }
    }
    return (graph, nodeMap)
}

fn create_risk_graph_tiled(data: Vec<Vec<u32>>) -> (DiGraph<(usize, usize), u32>, HashMap<(usize, usize), NodeIndex>) {
    let height = data.len();
    let width = data[0].len();
    let mut new_data: Vec<Vec<u32>> = vec![vec![0; width*5]; height*5];
    for row in 0..height {
        for col in 0..width {
            for tile_row in 0..5 {
                for tile_col in 0..5 {
                    let mut weight = data[row][col] + tile_row as u32 + tile_col as u32;
                    if weight > 9 {
                        weight = weight - 9;
                    }
                    assert_eq!(weight <= 9, true);
                    new_data[row + tile_row*height][col + tile_col*width] = weight;
                }
            }
        }
    }
    let mut graph = DiGraph::<(usize, usize), u32>::new();
    let new_height = new_data.len();
    let new_width = new_data[0].len();
    let mut nodeMap =  HashMap::<(usize, usize), NodeIndex>::new();
    // let mut edgeMap = HashMap::<((usize, usize), (usize, usize)), EdgeIndex>::new();
    // generate all nodes
    for row in 0..new_height {
        for col in 0..new_width {
            nodeMap.insert((row,col), graph.add_node((row, col)));
        }
    }
    // now add edges
    for row in 0..new_height {
        for col in 0..new_width {
            if row != 0 {
                graph.add_edge(nodeMap[&(row-1, col)], nodeMap[&(row, col)], new_data[row][col]);
            }
            if row+1 < new_height {
                graph.add_edge(nodeMap[&(row+1, col)], nodeMap[&(row, col)], new_data[row][col]);
            }
            if col != 0 {
                graph.add_edge(nodeMap[&(row, col-1)], nodeMap[&(row, col)], new_data[row][col]);
            }
            if col+1 < new_width {
                graph.add_edge(nodeMap[&(row, col+1)], nodeMap[&(row, col)], new_data[row][col]);
            }
        }
    }
    return (graph, nodeMap)
}

fn day15_part1(mut data: Vec<Vec<u32>>) -> u64 {
    let mut result = 0;
    let height = data.len();
    let width = data[0].len();
    let (risk_graph, nodes) = create_risk_graph(data);
    // println!("Graph: {:?}", risk_graph);
    let path = algo::astar(
        &risk_graph, 
        nodes[&(0,0)], 
         |n| n == nodes[&(height-1,width-1)], 
         |e| *e.weight(), 
         |_| 0
    ).unwrap();
    // println!("{:?}", path);
    return path.0 as u64;
}

fn day15_part2(mut data: Vec<Vec<u32>>) -> u64 {
    let mut result = 0;
    let height = data.len();
    let width = data[0].len();
    let (risk_graph, nodes) = create_risk_graph_tiled(data);
    // println!("Graph: {:?}", risk_graph);
    let path = algo::astar(
        &risk_graph, 
        nodes[&(0,0)], 
         |n| n == nodes[&(height*5-1,width*5-1)], 
         |e| *e.weight(), 
         |_| 0
    ).unwrap();
    // println!("{:?}", path);
    return path.0 as u64;
}
fn main() {
    println!("AOC2021 Day 15: Dijkstra's time");

    let data = read_input_file("./input_sample.txt");
    let result = day15_part1(data);
    println!("Day 15 Sample 1: Result: {}", result);

    let data = read_input_file("./input.txt");
    let result = day15_part1(data);
    println!("Day 15 Puzzle 1: Result: {}", result);

    let data = read_input_file("./input_sample.txt");
    let result = day15_part2(data);
    println!("Day 15 Sample 2: Result: {}", result);

    let data = read_input_file("./input.txt");
    let result = day15_part2(data);
    println!("Day 15 Puzzle 2: Result: {}", result);
}
