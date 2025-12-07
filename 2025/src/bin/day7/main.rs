use std::collections::{HashMap, HashSet};

/// Advent of Code 2022 - Day 14
/// https://adventofcode.com/2022/day/14
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, PartialOrd, Ord)]
struct Coords {
    row: usize,
    col: usize,
}

impl Coords {
    fn above(&self) -> Coords {
        Coords {
            col: self.col,
            row: self.row - 1,
        }
    }
    fn under(&self) -> Coords {
        Coords {
            col: self.col,
            row: self.row + 1,
        }
    }
    fn left(&self) -> Coords {
        Coords {
            col: self.col - 1,
            row: self.row,
        }
    }
    fn right(&self) -> Coords {
        Coords {
            col: self.col + 1,
            row: self.row,
        }
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut splits = 0;
    let mut beams: HashSet<Coords> = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let pos = Coords { row, col };
            match ch {
                'S' => {
                    let _ = beams.insert(pos.under());
                }
                '^' => {
                    if let Some(_) = beams.get(&pos.above()) {
                        beams.remove(&pos);
                        let _ = beams.insert(pos.left());
                        let _ = beams.insert(pos.left().under());
                        let _ = beams.insert(pos.right());
                        let _ = beams.insert(pos.right().under());
                        splits += 1;
                    }
                }
                _ => {
                    if let Some(beam) = beams.get(&pos) {
                        let _ = beams.insert(beam.under());
                    }
                }
            }
        }
    }

    println!("{splits}")
}

fn timelines(
    current_beams: &HashSet<Coords>,
    current_processing_pos: Coords,
    grid: &Vec<Vec<char>>,
    memo: &mut HashMap<(Vec<Coords>, Coords), u32>,
) -> u32 {
    todo!()
}

#[allow(unused)]
fn part2(input: &str) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_row = grid.len();
    let max_col = grid[0].len();

    let mut start_s_pos = Coords { row: 0, col: 0 };
    'outer: for r in 0..max_row {
        for c in 0..max_col {
            if grid[r][c] == 'S' {
                start_s_pos = Coords { row: r, col: c };
                break 'outer;
            }
        }
    }

    let init_beam: HashSet<Coords> = vec![start_s_pos.under()].into_iter().collect();
    let mut memo = HashMap::new();

    let total_timelines = timelines(&init_beam, Coords { row: 0, col: 0 }, &grid, &mut memo);

    println!("{}", total_timelines + 1);
}
