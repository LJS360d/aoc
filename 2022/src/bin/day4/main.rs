/// Advent of Code 2022 - Day 4
/// https://adventofcode.com/2022/day/4
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let (e1, e2) = line.split_once(',').unwrap();
        let (e1s1, e1s2) = e1.split_once('-').unwrap();
        let (e2s1, e2s2) = e2.split_once('-').unwrap();
        let mut chars = line.chars();
    }
}
