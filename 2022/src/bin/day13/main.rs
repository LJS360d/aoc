use std::num::ParseIntError;

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
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let mut chars = s.chars().peekable();
        if chars.next() != Some('[') {
            panic!("Input must start with '['");
        }

        let result_list = Self::parse_list(&mut chars)?;

        Ok(Packet::List(result_list))
    }

    fn parse_list<I>(chars: &mut std::iter::Peekable<I>) -> Result<Vec<Packet>, ParseIntError>
    where
        I: Iterator<Item = char>,
    {
        let mut list = Vec::new();

        loop {
            while let Some(',') = chars.peek() {
                chars.next();
            }

            // Check for the closing bracket first
            match chars.peek() {
                Some(']') => {
                    chars.next(); // Consume the ']'
                    return Ok(list);
                }
                Some('[') => {
                    chars.next(); // Consume the '['
                    let sub_list = Self::parse_list(chars)?;
                    list.push(Packet::List(sub_list));
                }
                Some(c) if c.is_digit(10) => {
                    // Parse a number
                    let mut num_str = String::new();
                    while let Some(c) = chars.peek() {
                        if c.is_digit(10) {
                            num_str.push(*c);
                            chars.next(); // Consume the digit
                        } else {
                            break;
                        }
                    }
                    let value = num_str.parse()?;
                    list.push(Packet::Value(value));
                }
                None => panic!("Unexpected end of input (missing ']')"),
                Some(c) => panic!("Unexpected character: {}", c),
            }
        }
    }

    pub fn is_smaller_than(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::Value(v1), Packet::Value(v2)) => return v1 < v2,
            (Packet::List(l1), Packet::List(l2)) => {
                let mut l1_iter = l1.iter();
                let mut l2_iter = l2.iter();
                loop {
                    let pc1 = l1_iter.next();
                    let pc2 = l2_iter.next();

                    if pc1.is_none() && pc2.is_none() {
                        return false;
                    }

                    if pc1.is_none() {
                        return true;
                    }
                    if pc2.is_none() {
                        return false;
                    }
                    let pc1 = pc1.unwrap();
                    let pc2 = pc2.unwrap();

                    if pc1.is_smaller_than(pc2) {
                        return true;
                    } else {
                        // if pc2 is smaller than pc1 we need to return false
                        if pc2.is_smaller_than(pc1) {
                            return false;
                        }
                    }
                }
            }
            (Packet::List(_), Packet::Value(v2)) => {
                let l2 = Packet::List(vec![Packet::Value(*v2)]);
                return self.is_smaller_than(&l2);
            }
            (Packet::Value(v1), Packet::List(_)) => {
                let l1 = Packet::List(vec![Packet::Value(*v1)]);
                return l1.is_smaller_than(other);
            }
        }
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut count = 0;
    for (i, pairs) in input.split("\n\n").enumerate() {
        let (p1_str, p2_str) = pairs.split_once('\n').unwrap();
        let p1 = Packet::from_str(p1_str).unwrap();
        let p2 = Packet::from_str(p2_str).unwrap();

        if p1.is_smaller_than(&p2) {
            count += (i + 1);
        }
    }
    println!("{}", count);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
