/// Advent of Code 2022 - Day 3
/// https://adventofcode.com/2022/day/3
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

fn get_priority(item_type: char) -> u8 {
    // Cast the character to its ASCII value (u8 is sufficient for standard ASCII)
    let ascii_value = item_type as u8;

    match item_type {
        // Lowercase 'a' through 'z' (Priority 1-26)
        'a'..='z' => {
            // 'a' (97) - 96 = 1
            // 'z' (122) - 96 = 26
            ascii_value - b'a' + 1
        }
        // Uppercase 'A' through 'Z' (Priority 27-52)
        'A'..='Z' => {
            // 'A' (65) - 64 = 1. But we need 27.
            // (65 - 'A' + 27) -> (65 - 65 + 27) = 27
            ascii_value - b'A' + 27
        }
        _ => 0,
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut sum = 0;
    for line in lines {
        let len = line.len();
        let prefix = line.chars().take(len / 2).collect::<String>();
        let suffix = line.chars().rev().take(len / 2).collect::<String>();

        for item in suffix.chars() {
            if prefix.contains(item) {
                sum += get_priority(item) as u32;
                break;
            }
        }
    }
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut groups: Vec<[&str; 3]> = vec![];
    for (i, line) in lines.enumerate() {
        if i % 3 == 0 {
            groups.push(["", "", ""]);
        }
        groups.last_mut().unwrap()[i % 3] = line;
    }

    let mut sum = 0;
    for group in groups {
        for char in group[0].chars() {
            if group[1].contains(char) && group[2].contains(char) {
                sum += get_priority(char) as u32;
                break;
            }
        }
    }

    println!("{}", sum);
}
