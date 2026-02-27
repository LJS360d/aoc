/// Advent of Code 2021 - Day 2
/// https://adventofcode.com/2021/day/2
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let mut depth = 0;
    let mut h = 0;
    for line in input.lines() {
        let arr: Vec<&str> = line.split(" ").collect();
        let (dir, amount_str) = (arr[0], arr[1]);
        let amount: i32 = amount_str.parse().unwrap();
        match dir {
            "forward" => {
                h = h + amount;
            }
            "up" => {
                depth = depth - amount;
            }
            "down" => {
                depth = depth + amount;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", h * depth);
}

#[allow(unused)]
fn part2(input: &str) {
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    let mut h: i64 = 0;
    for line in input.lines() {
        let arr: Vec<&str> = line.split(" ").collect();
        let (dir, amount_str) = (arr[0], arr[1]);
        let amount: i64 = amount_str.parse().unwrap();
        match dir {
            "forward" => {
                h = h + amount;
                depth = depth + aim * amount;
            }
            "up" => {
                aim = aim - amount;
            }
            "down" => {
                aim = aim + amount;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", h * depth);
}
