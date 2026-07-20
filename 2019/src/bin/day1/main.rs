/// Advent of Code 2019 - Day 1
/// https://adventofcode.com/2019/day/1
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let n = input
        .lines()
        .map(|line| {
            let num = line.parse::<f32>().unwrap();
            let f: f32 = num / 3_f32;
            let g: i32 = f.floor() as i32;
            return g - 2;
        })
        .sum::<i32>();
    println!("{}", n)
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
