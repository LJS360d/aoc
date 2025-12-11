use std::{cmp::max, collections::HashSet};

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

#[allow(unused)]
fn part2(input: &str) {
    let grid_height = input.lines().count();
    let grid_width = input.lines().next().unwrap().len();
    let mut grid: Vec<Vec<u64>> = Vec::new();
    let mut default_col = Vec::with_capacity(grid_width);
    default_col.resize(grid_width, 0);
    grid.resize(grid_height, default_col);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                grid[y][x] = 1;
                continue;
            }

            if ch == '.' && y as isize - 1 >= 0 {
                grid[y][x] += grid[y - 1][x];
                continue;
            }

            if ch == '^' {
                for j in [-1_isize, 1_isize] {
                    let new_col = (x as isize + j) as usize;
                    grid[y][new_col] = max(grid[y - 1][x] + grid[y][new_col], grid[y][new_col]);
                }
            }
        }
    }

    println!("{}", grid[grid_height - 1].iter().sum::<u64>())
}
