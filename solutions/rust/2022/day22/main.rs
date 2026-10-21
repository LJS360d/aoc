use std::collections::{HashMap, HashSet};

use regex::Regex;

/// Advent of Code 2022 - Day 22
/// https://adventofcode.com/2022/day/22
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let last_line = lines.clone().last().unwrap();
    let mut map = HashMap::<(i32, i32), char>::new();
    let mut pos: (i32, i32) = (0, 0);
    // 0 -> Right
    // 1 -> Down
    // 2 -> Left
    // 3 -> Up
    let mut dir = 0;
    for (row, line) in lines.enumerate() {
        if line.trim().is_empty() {
            break;
        }
        let mut chars = line.chars();
        for (col, ch) in chars.enumerate() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => continue,
                _ => {
                    if map.len() == 0 {
                        pos = (row as i32 + 1, col as i32 + 1)
                    }
                    map.insert((row as i32 + 1, col as i32 + 1), ch);
                }
            }
        }
    }
    let re = Regex::new(r"(\d+)|([A-Z]+)").unwrap();
    for capture in re.captures_iter(last_line) {
        match capture.get_match().as_str() {
            "R" => dir = (dir + 1) % 4,
            "L" => {
                if dir == 0 {
                    dir = 3
                } else {
                    dir = (dir - 1)
                }
            }
            n => {
                let num = n.parse::<u32>().unwrap();
                'outer: for _ in 0..num {
                    let next_pos = match dir {
                        0 => (pos.0, pos.1 + 1),
                        1 => (pos.0 + 1, pos.1),
                        2 => (pos.0, pos.1 - 1),
                        3 => (pos.0 - 1, pos.1),
                        _ => unreachable!(),
                    };
                    match map.get(&next_pos) {
                        Some(ch) => match ch {
                            '.' => {
                                pos = next_pos;
                                println!("{},{}", pos.0, pos.1);
                            }
                            '#' => break 'outer,
                            _ => unreachable!(),
                        },
                        None => {
                            // wrap around
                            match dir {
                                0 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.0 == pos.0)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.1, |init, &p| {
                                        if p.1 < init { p.1 } else { init }
                                    });
                                    let wrap_coords = (pos.0, wrap_coord);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                1 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.1 == pos.1)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.0, |init, &p| {
                                        if p.0 < init { p.0 } else { init }
                                    });
                                    let wrap_coords = (wrap_coord, pos.1);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                2 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.0 == pos.0)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.1, |init, &p| {
                                        if p.1 > init { p.1 } else { init }
                                    });
                                    let wrap_coords = (pos.0, wrap_coord);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                3 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.1 == pos.1)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.0, |init, &p| {
                                        if p.0 > init { p.0 } else { init }
                                    });
                                    let wrap_coords = (wrap_coord, pos.1);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                _ => unreachable!(),
                            }
                            println!("{},{}", pos.0, pos.1);
                        }
                    }
                }
            }
        }
    }
    println!("{},{} {}", pos.0, pos.1, dir);
    println!("{}", 1000 * pos.0 + 4 * pos.1 + dir)
}

type Coords = (i32, i32);

#[derive(Eq, Hash, PartialEq)]
struct Node {
    pos: Coords,
    val: char,
}

struct Edge {
    src: Coords,
    trg: Coords,
    dir: i32,
}

struct Graph {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let last_line = lines.clone().last().unwrap();
    let mut graph = Graph::new();
    const BOUND: i32 = 50;
    let mut pos = (0, 0);
    // Directions:
    // 0 -> Right
    // 1 -> Down
    // 2 -> Left
    // 3 -> Up
    let mut dir = 0;
    for (row, line) in lines.enumerate() {
        if line.trim().is_empty() {
            break;
        }
        let mut chars = line.chars();
        for (col, ch) in chars.enumerate() {
            match ch {
                ' ' | '\n' | '\t' | '\r' => continue,
                '.' => {
                    if graph.nodes.len() == 0 {
                        pos = (row as i32 + 1, col as i32 + 1)
                    }
                    let node_pos = (row as i32 + 1, col as i32 + 1);
                    graph.nodes.insert(Node {
                        pos: node_pos,
                        val: ch,
                    });
                    let right = (node_pos.0, node_pos.1 + 1);
                    match chars.nth(col + 1) {
                        Some(r) => {
                            match r {
                                '#' => continue,
                                '.' => {
                                    graph.edges.insert(Edge{
                                        dir: 0,

                                    })
                                }
                            }
                        },
                        None => {
                            // 2->5
                            //
                        }
                    }
                    let left = (node_pos.0, node_pos.1 - 1);
                    let down = (node_pos.0 + 1, node_pos.1);
                    let up = (node_pos.0 - 1, node_pos.1);
                }
            }
        }
    }
    let re = Regex::new(r"(\d+)|([A-Z]+)").unwrap();
    for capture in re.captures_iter(last_line) {
        match capture.get_match().as_str() {
            "R" => dir = (dir + 1) % 4,
            "L" => {
                if dir == 0 {
                    dir = 3
                } else {
                    dir = (dir - 1)
                }
            }
            n => {
                let num = n.parse::<u32>().unwrap();
                'outer: for _ in 0..num {
                    let next_pos = match dir {
                        0 => (pos.0, pos.1 + 1),
                        1 => (pos.0 + 1, pos.1),
                        2 => (pos.0, pos.1 - 1),
                        3 => (pos.0 - 1, pos.1),
                        _ => unreachable!(),
                    };
                    match map.get(&next_pos) {
                        Some(ch) => match ch {
                            '.' => {
                                pos = next_pos;
                                println!("{},{}", pos.0, pos.1);
                            }
                            '#' => break 'outer,
                            _ => unreachable!(),
                        },
                        None => {
                            // wrap around
                            match dir {
                                0 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.0 == pos.0)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.1, |init, &p| {
                                        if p.1 < init { p.1 } else { init }
                                    });
                                    let wrap_coords = (pos.0, wrap_coord);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                1 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.1 == pos.1)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.0, |init, &p| {
                                        if p.0 < init { p.0 } else { init }
                                    });
                                    let wrap_coords = (wrap_coord, pos.1);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                2 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.0 == pos.0)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.1, |init, &p| {
                                        if p.1 > init { p.1 } else { init }
                                    });
                                    let wrap_coords = (pos.0, wrap_coord);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                3 => {
                                    let positions = map
                                        .keys()
                                        .filter(|&p| p.1 == pos.1)
                                        .collect::<Vec<&(i32, i32)>>();
                                    let wrap_coord = positions.iter().fold(pos.0, |init, &p| {
                                        if p.0 > init { p.0 } else { init }
                                    });
                                    let wrap_coords = (wrap_coord, pos.1);
                                    match map.get(&wrap_coords) {
                                        Some('.') => pos = wrap_coords,
                                        _ => break,
                                    }
                                }
                                _ => unreachable!(),
                            }
                            println!("{},{}", pos.0, pos.1);
                        }
                    }
                }
            }
        }
    }
    println!("{},{} {}", pos.0, pos.1, dir);
    println!("{}", 1000 * pos.0 + 4 * pos.1 + dir)
}
