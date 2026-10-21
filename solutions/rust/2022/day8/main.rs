use std::{collections::HashMap, vec};

/// Advent of Code 2022 - Day 8
/// https://adventofcode.com/2022/day/8
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in lines {
        grid.push(
            line.chars()
                .map(|a| a.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut visible_trees: HashMap<String, u32> = HashMap::<String, u32>::new();

    for (col, iter) in grid.iter().enumerate() {
        for (row, value) in iter.iter().enumerate() {
            let key = format!("{col};{row}");

            if col == 0 || row == 0 || row == (iter.len() - 1) || col == (grid.len() - 1) {
                visible_trees.insert(key, *value);
                continue;
            }

            let mut is_visible = true;
            // view from top
            for prev in (0..col).collect::<Vec<usize>>().iter().rev() {
                let prev_tree = grid[*prev][row];
                if prev_tree >= *value {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_trees.insert(key, *value);
                continue;
            }
            is_visible = true;
            // view from bottom
            for next in (col + 1..grid.len()).collect::<Vec<usize>>().iter() {
                let next_tree = grid[*next][row];
                if next_tree >= *value {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_trees.insert(key, *value);
                continue;
            }
            is_visible = true;
            // view from left
            for prev in (0..row).collect::<Vec<usize>>().iter().rev() {
                let prev_tree = grid[col][*prev];
                if prev_tree >= *value {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_trees.insert(key, *value);
                continue;
            }
            is_visible = true;
            // view from right
            for next in (row + 1..iter.len()).collect::<Vec<usize>>().iter() {
                let next_tree = grid[col][*next];
                if next_tree >= *value {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_trees.insert(key, *value);
                continue;
            }
        }
    }

    println!("{}", visible_trees.len());
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in lines {
        grid.push(
            line.chars()
                .map(|a| a.to_digit(10).unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    let mut highest_score = 0;

    for (col, iter) in grid.iter().enumerate() {
        for (row, value) in iter.iter().enumerate() {
            // let key = format!("{col};{row}");

            let mut top_score = 0;
            // view from top
            for prev in (0..col).collect::<Vec<usize>>().iter().rev() {
                let prev_tree = grid[*prev][row];
                top_score += 1;
                if prev_tree >= *value {
                    break;
                }
            }
            let mut bottom_score = 0;
            // view from bottom
            for next in (col + 1..grid.len()).collect::<Vec<usize>>().iter() {
                let next_tree = grid[*next][row];
                bottom_score += 1;
                if next_tree >= *value {
                    break;
                }
            }
            let mut left_score = 0;
            // view from left
            for prev in (0..row).collect::<Vec<usize>>().iter().rev() {
                let prev_tree = grid[col][*prev];
                left_score += 1;
                if prev_tree >= *value {
                    break;
                }
            }
            let mut right_score = 0;
            // view from right
            for next in (row + 1..iter.len()).collect::<Vec<usize>>().iter() {
                let next_tree = grid[col][*next];
                right_score += 1;
                if next_tree >= *value {
                    break;
                }
            }

            let score = top_score * bottom_score * left_score * right_score;
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("{}", highest_score);
}
