use std::collections::HashSet;

/// Advent of Code 2021 - Day 5
/// https://adventofcode.com/2021/day/5
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let lines: Vec<(usize, usize, usize, usize)> = input
        .lines()
        .map(|l| {
            let (_, [x1, y1, x2, y2]) = re.captures(l).unwrap().extract();
            let x1 = x1.parse::<usize>().unwrap();
            let y1 = y1.parse::<usize>().unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();
            (x1, y1, x2, y2)
        })
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect();
    let mut doubles: HashSet<(usize, usize)> = HashSet::new();
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    for pair in lines {
        let c = get_straight_coords(&pair);
        for coord in c.iter() {
            if !coords.insert(*coord) {
                doubles.insert(*coord);
            }
        }
    }
    println!("{:?}", doubles.len());
}

fn get_straight_coords((x1, y1, x2, y2): &(usize, usize, usize, usize)) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = vec![];
    if x1 == x2 {
        for y in *y1.min(y2)..=*y2.max(y1) {
            coords.push((*x1, y));
        }
    } else if y1 == y2 {
        for x in *x1.min(x2)..=*x2.max(x1) {
            coords.push((x, *y1));
        }
    }
    coords
}

#[allow(unused)]
fn part2(input: &str) {
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let lines: Vec<(usize, usize, usize, usize)> = input
        .lines()
        .map(|l| {
            let (_, [x1, y1, x2, y2]) = re.captures(l).unwrap().extract();
            let x1 = x1.parse::<usize>().unwrap();
            let y1 = y1.parse::<usize>().unwrap();
            let x2 = x2.parse::<usize>().unwrap();
            let y2 = y2.parse::<usize>().unwrap();
            (x1, y1, x2, y2)
        })
        .collect();
    let mut doubles: HashSet<(usize, usize)> = HashSet::new();
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    for pair in lines {
        let c = get_all_coords(&pair);
        for coord in c.iter() {
            if !coords.insert(*coord) {
                doubles.insert(*coord);
            }
        }
    }
    println!("{:?}", doubles.len());
}

fn get_all_coords((x1, y1, x2, y2): &(usize, usize, usize, usize)) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = vec![];
    let x_min = *x1.min(x2);
    let x_max = *x1.max(x2);
    let y_min = *y1.min(y2);
    let y_max = *y1.max(y2);
    if x1 == x2 {
        for y in y_min..=y_max {
            coords.push((*x1, y));
        }
    } else if y1 == y2 {
        for x in x_min..=x_max {
            coords.push((x, *y1));
        }
    } else {
        // diagonal
        let x_diff = x_max - x_min;
        let y_diff = y_max - y_min;
        let diff = x_diff.max(y_diff);
        for d in 0..=diff {
            let x = x_min + d;
            let y = y_min + d;
            coords.push((x, y));
        }
    }
    coords
}
