use std::collections::HashMap;

use regex::Regex;

/// Advent of Code 2025 - Day 8
/// https://adventofcode.com/2025/day/8
fn main() {
    let input = include_str!("./test_input.txt");
    part1(&input);
    // part2(&input);
}

type Point = (u32, u32, u32);

fn euclidean_distance(p1: Point, p2: Point) -> u32 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs() + (z1 as i32 - z2 as i32).abs())
        as u32
}

#[allow(unused)]
fn part1(input: &str) {
    let mut edges: Vec<(Point, Point)> = Vec::new();
    let mut nodes = Vec::<Point>::new();
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    for line in input.lines() {
        let (_, [x_str, y_str, z_str]) = re.captures(line).unwrap().extract::<3>();
        let x = x_str.parse::<u32>().unwrap();
        let y = y_str.parse::<u32>().unwrap();
        let z = z_str.parse::<u32>().unwrap();
        nodes.push((x, y, z));
    }
    let mut adj_list: HashMap<(Point, Point), u32> = HashMap::new();
    for &n1 in nodes.iter() {
        for &n2 in nodes.iter() {
            let _ = adj_list.insert((n1, n2), euclidean_distance(n1, n2));
        }
    }
    // pick the 2 closest nodes
    for ((p1, p2), dist) in adj_list.iter() {}
    // add an edge between them
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
