/// Advent of Code 2021 - Day 1
/// https://adventofcode.com/2021/day/1
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let depths: Vec<i32> = lines.map(|line| line.parse().unwrap()).collect();
    let mut c = 0;
    for i in 1..depths.len() {
        let Some(prev) = depths.get(i - 1) else {
            continue;
        };
        let Some(curr) = depths.get(i) else {
            continue;
        };
        if prev < curr {
            c = c + 1;
        }
    }
    println!("{}", c);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut depths: Vec<i32> = vec![];

    for i in 0..lines.len() {
        let curr: i32 = lines[i].parse().unwrap();
        let Some(next_str) = lines.get(i + 1) else {
            continue;
        };
        let Some(next_next_str) = lines.get(i + 2) else {
            continue;
        };

        let next: i32 = next_str.parse().unwrap();
        let next_next: i32 = next_next_str.parse().unwrap();

        depths.push(curr + next + next_next);
    }

    let mut c = 0;
    for i in 1..depths.len() {
        let Some(prev) = depths.get(i - 1) else {
            continue;
        };
        let Some(curr) = depths.get(i) else {
            continue;
        };
        if prev < curr {
            c = c + 1;
        }
    }
    println!("{}", c);
}
