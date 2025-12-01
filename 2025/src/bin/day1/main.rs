/// Advent of Code 2025 - Day 1
/// https://adventofcode.com/2025/day/1
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut pos = 50;
    let mut counter = 0;
    for line in lines {
        let (dir, amount_str) = line.split_at(1);
        let amount = amount_str.parse::<u32>().unwrap();
        match dir {
            "L" => {
                if (amount % 100) > pos {
                    pos += 100 - (amount % 100);
                } else {
                    pos -= amount % 100
                }
            }
            "R" => {
                pos += amount;
                pos %= 100
            }
            _ => {}
        }
        if pos == 0 {
            counter += 1;
        }
    }
    println!("{counter}");
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let mut pos = 50;
    let mut counter = 0;

    for line in lines {
        let (dir, amount_str) = line.split_at(1);
        let amount = amount_str.parse::<u32>().unwrap();

        let prev_pos = pos;

        match dir {
            "L" => {
                if prev_pos == 0 {
                    counter += amount / 100;
                } else {
                    if amount >= prev_pos {
                        counter += (amount - prev_pos) / 100 + 1;
                    }
                }
                pos = (prev_pos as i32 - amount as i32).rem_euclid(100) as u32;
            }
            "R" => {
                counter += (prev_pos + amount) / 100;
                pos = (prev_pos + amount) % 100;
            }
            _ => {}
        }
    }
    println!("{counter}");
}
