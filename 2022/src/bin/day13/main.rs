use std::num::{IntErrorKind, ParseIntError};

/// Advent of Code 2022 - Day 13
/// https://adventofcode.com/2022/day/13
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl Packet {
    fn from_str(s: &str, stack: &mut Vec<Packet>) -> Result<Self, ParseIntError> {
        let mut chars = s.chars();

        while let Some(c) = &chars.next() {
            println!("chars left {chars:?}");

            match c {
                '[' => {
                    let mut list = Vec::new();
                    while let Ok(packet) = Packet::from_str(&chars.as_str(), stack) {
                        list.push(packet);
                        let _ = &chars.next();
                    }
                    stack.push(Packet::List(list));
                }
                ']' => return Ok(Packet::List(stack.clone())),
                ',' => continue,
                _ => {
                    println!("proc {c}");
                    let mut num = c.to_string();
                    // clone to peek into the next chars without consuming
                    while let Some(next_char) = chars.clone().next() {
                        if next_char.is_digit(10) {
                            num.push(next_char);
                            // consume now
                            let _ = &chars.next();
                        } else {
                            break;
                        }
                    }
                    println!("num: {num}");
                    stack.push(Packet::Value(num.parse().unwrap()));
                }
            }
        }
        Ok(Packet::List(stack.clone()))
    }
}

#[allow(unused)]
fn part1(input: &str) {
    for (i, pairs) in input.split("\n\n").enumerate() {
        let (p1_str, p2_str) = pairs.split_once('\n').unwrap();
        let p1 = Packet::from_str(p1_str, &mut Vec::new()).unwrap();
        let p2 = Packet::from_str(p2_str, &mut Vec::new()).unwrap();
        println!("== Pair {i} ==");
        println!("- Compare {p1:?} vs {p2:?}");
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
