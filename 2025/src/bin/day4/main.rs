use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Advent of Code 2025 - Day 4
/// https://adventofcode.com/2025/day/4
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

type Coords = (i32, i32);

struct Grid {
    data: HashMap<Coords, char>,
}

impl Grid {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn identify_adjacent(&self, coords: Coords) -> (Vec<Coords>, Vec<Coords>) {
        let mut blocked = vec![];
        let mut free = vec![];
        let valid_adj: Vec<Coords> = vec![
            (coords.0, coords.1 + 1),     // right
            (coords.0 + 1, coords.1),     // down
            (coords.0, coords.1 - 1),     // left
            (coords.0 - 1, coords.1),     // up
            (coords.0 + 1, coords.1 + 1), // down-right
            (coords.0 + 1, coords.1 - 1), // down-left
            (coords.0 - 1, coords.1 - 1), // up-left
            (coords.0 - 1, coords.1 + 1), // up-right
        ];

        for valid_pos in valid_adj {
            if let Some(val) = self.get(&valid_pos) {
                match val {
                    '@' => {
                        blocked.push(valid_pos);
                    }
                    '.' => {
                        free.push(valid_pos);
                    }
                    _ => {}
                }
            }
        }

        (blocked, free)
    }
}

impl Deref for Grid {
    type Target = HashMap<Coords, char>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut grid = Grid::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let None = grid.insert((row as i32, col as i32), char) else {
                panic!("{:?} already present", (row, col))
            };
        }
    }
    let mut accessible = vec![];
    for (coords, value) in grid.iter() {
        if *value != '@' {
            continue;
        }
        let (blocked, free) = grid.identify_adjacent(*coords);
        if blocked.len() < 4 {
            accessible.push(coords);
        }
    }
    println!("{}", accessible.len());
}

#[allow(unused)]
fn part2(input: &str) {
    let mut grid = Grid::new();

    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let None = grid.insert((row as i32, col as i32), char) else {
                panic!("{:?} already present", (row, col))
            };
        }
    }
    let mut sum = 0;
    loop {
        let mut accessible = vec![];
        for (coords, value) in grid.iter() {
            if *value != '@' {
                continue;
            }
            let (blocked, free) = grid.identify_adjacent(*coords);
            if blocked.len() < 4 {
                accessible.push(coords.clone());
            }
        }
        if accessible.len() == 0 {
            break;
        }
        for acc in accessible.iter() {
            let _ = grid.insert(*acc, '.');
        }
        sum += accessible.len()
    }
    println!("{}", sum);
}
