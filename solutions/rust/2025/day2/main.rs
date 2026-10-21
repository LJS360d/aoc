/// Advent of Code 2025 - Day 2
/// https://adventofcode.com/2025/day/2
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let mut sum = 0;
    let ranges = input.split(",").for_each(|str| {
        let (left_str, right_str) = str.split_once("-").unwrap();
        let left = left_str.parse::<u64>().unwrap();
        let right = right_str.parse::<u64>().unwrap();
        for id in left..=right {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            let (id_pre, id_suf) = id_str.split_at(id_str.len() / 2);
            if id_pre == id_suf {
                sum += id;
            }
        }
    });
    println!("{sum}")
}

#[allow(unused)]
fn part2(input: &str) {
    let mut sum = 0;
    let ranges = input.split(",").for_each(|str| {
        let (left_str, right_str) = str.split_once("-").unwrap();
        let left = left_str.parse::<u64>().unwrap();
        let right = right_str.parse::<u64>().unwrap();
        for id in left..=right {
            let id_str = id.to_string();
            for i in 1..=(id_str.len() / 2) {
                if id_str.len() % i != 0 {
                    continue;
                }
                let vec = split_at_multiple_of(id_str.as_str(), i);
                let first = vec.first().unwrap();
                if vec.iter().all(|s| s == first) {
                    sum += id;
                    break;
                }
            }
        }
    });
    println!("{sum}")
}

fn split_at_multiple_of(str: &str, every: usize) -> Vec<&str> {
    if every == 0 {
        panic!("every must be greater than 0");
    }
    let mut result = Vec::new();
    let mut i = every;
    while i <= str.len() {
        result.push(&str[i - every..i]);
        i += every;
    }
    result
}
