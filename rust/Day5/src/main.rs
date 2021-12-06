use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x: x,
            y: y
        }
    }
    pub fn print(&self) {
        print!("Point @{:p}: (", self);
        print!("x: {}, y: {} )", self.x, self.y);
    }
    pub fn approach(self, target: Point) -> Point {
        // approach a target from current point, returns the next closest point
        let line_seg = LineSegment::new(self, target);
        assert_eq!(line_seg.is_axial() || line_seg.is_diagonal(), true);
        // 0,0 -> 2,2
        let x_step: i32;
        let y_step: i32;
        if self.x == target.x {
            x_step = 0;
        } else if self.x < target.x {
            x_step = 1;
        } else {
            x_step = -1;
        }
        if self.y == target.y {
            y_step = 0;
        } else if self.y < target.y {
            y_step = 1;
        } else {
            y_step = -1;
        }
        return Point::new(self.x + x_step, self.y + y_step);
    }
}

#[derive(Clone, Copy)]
struct LineSegment {
    p1: Point,
    p2: Point
}

impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> Self {
        LineSegment {
            p1: p1,
            p2: p2
        }
    }
    pub fn print(&self) {
        print!("LineSegment @{:p}: ", self);
        print!("p1: ");
        self.p1.print();
        print!(", p2: ");
        self.p2.print();
        println!("")
    }
    pub fn is_axial(&self) -> bool {
        if (self.p1.x == self.p2.x) || (self.p1.y == self.p2.y) {
            return true;
        }
        return false;
    }
    pub fn is_diagonal(&self) -> bool {
        let x_delta = (self.p1.x - self.p2.x).abs();
        let y_delta = (self.p1.y - self.p2.y).abs();
        if x_delta == y_delta {
            return true;
        }
        return false;
    }
    pub fn get_points(&self) -> Vec<Point> {
        // return a vector of points in the line (needs to be axial or diagonal)
        let mut points: Vec<Point> = vec![];
        if self.is_axial() || self.is_diagonal() {
            points.push(self.p1);
            let destination_point: Point = self.p2;
            let mut current_point: Point = self.p1;
            while current_point != destination_point {
                current_point = current_point.approach(destination_point);
                points.push(current_point);
            }
        } else {
            panic!("Line should be axial or diagonal to traverse")
        }
        return points;
    }
}

fn day5_part1(data: &Vec<LineSegment>) -> i32 {
    let mut active_points = HashMap::<Point, u32>::new();
    let mut overlapped_points = 0;
    for line_seg in data {
        // line_seg.print();
        if line_seg.is_axial() {
            // println!("Line is axial - traversing.");
            for point in line_seg.get_points() {
                let count = active_points.entry(point).or_insert(0);
                *count += 1;
                if *count == 2 {
                    overlapped_points += 1;
                    // println!("Overlap detected: ");
                    // point.print();
                    // println!("")
                }
            }
        }
    }
    return overlapped_points;
}

fn day5_part2(data: &Vec<LineSegment>) -> i32 {
    let mut active_points = HashMap::<Point, u32>::new();
    let mut overlapped_points = 0;
    for line_seg in data {
        // line_seg.print();
        if line_seg.is_axial() || line_seg.is_diagonal() {
            // println!("Line is axial - traversing.");
            for point in line_seg.get_points() {
                let count = active_points.entry(point).or_insert(0);
                *count += 1;
                if *count == 2 {
                    overlapped_points += 1;
                    // println!("Overlap detected: ");
                    // point.print();
                    // println!("")
                }
            }
        }
    }
    return overlapped_points;
}

fn read_line_file(path_string: &'static str) -> Vec<LineSegment> {
    let path = Path::new(path_string);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    // parse the list of lines
    // let draw_order: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut line_segments = vec![];
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        assert_eq!(2, points.len());
        let p1_vec: Vec<i32> = points[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let p1: Point = Point::new(p1_vec[0], p1_vec[1]);
        let p2_vec: Vec<i32> = points[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let p2: Point = Point::new(p2_vec[0], p2_vec[1]);
        let line_seg: LineSegment = LineSegment::new(p1, p2);
        line_segments.push(line_seg);
    }
    return line_segments;
}

fn main() {
    println!("AOC2021 Day 5: lines --------------- lines");

    let data = read_line_file("./input_sample.txt");
    let result = day5_part1(&data);
    println!("Day 5 Sample 1: Overlapping Points: {}", result);
    assert_eq!(5, result);

    let data = read_line_file("./input.txt");
    let result = day5_part1(&data);
    println!("Day 5 Puzzle 1: Overlapping Points: {}", result);
    assert_eq!(4745, result);

    let data = read_line_file("./input_sample.txt");
    let result = day5_part2(&data);
    println!("Day 5 Sample 2: Overlapping Points: {}", result);
    assert_eq!(12, result);

    let data = read_line_file("./input.txt");
    let result = day5_part2(&data);
    println!("Day 5 Puzzle 1: Overlapping Points: {}", result);
    assert_eq!(18442, result);
}
