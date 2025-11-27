use std::{thread::sleep, time::Duration};

/// Advent of Code 2022 - Day 14
/// https://adventofcode.com/2022/day/14
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

fn visualize(rocks: &Vec<(i32, i32)>, sand_origin: &(i32, i32), sand: &Vec<(i32, i32)>) {
    let mut min_bounds = rocks
        .iter()
        .fold((i32::MAX, i32::MAX), |(min_x, min_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y))
        });
    let max_bounds = rocks.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });
    min_bounds.1 = 0;

    let width = max_bounds.0 - min_bounds.0 + 1;
    let height = max_bounds.1 - min_bounds.1 + 1;

    print!("\x1b[2J\x1b[H");
    for y in -1..height + 1 {
        for x in -1..width + 1 {
            let c = (min_bounds.0 + x, min_bounds.1 + y);
            let at_sand_origin = c == *sand_origin;
            let at_rock = rocks.iter().any(|(rx, ry)| *rx == c.0 && *ry == c.1);
            let at_sand = sand.iter().any(|(rx, ry)| *rx == c.0 && *ry == c.1);

            if at_rock {
                print!("#");
            } else if at_sand_origin {
                print!("+");
            } else if at_sand {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn step(obs: Vec<(i32, i32)>, start_pos: &(i32, i32)) -> Option<(i32, i32)> {
    // apply gravity until an obstacle is found -> same X as curr_pos, higher Y
    let next_obs = match obs.iter().try_fold((0, 0), |(x, y)| *x == start_pos.0) {
        Some(i) => i,
        None => return None,
    };
    let down_left = (next_obs.0 - 1, next_obs.1);
    if obs
        .iter()
        .any(|(x, y)| *x == down_left.0 && *y == down_left.1)
    {
        return Some(down_left);
    }
    let down_right = (next_obs.0 + 1, next_obs.1);
    if obs
        .iter()
        .any(|(x, y)| *x == down_right.0 && *y == down_right.1)
    {
        return Some(down_right);
    }
    return Some((next_obs.0, next_obs.1 - 1));
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut rocks: Vec<(i32, i32)> = vec![];
    for line in lines {
        let mut markers = line.split(" -> ").map(|s| {
            s.split_once(",")
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap()
        });

        let mut curr = markers.next().unwrap();
        rocks.push(curr);
        loop {
            let next = match markers.next() {
                Some(n) => n,
                None => break,
            };

            if curr.0 == next.0 {
                let go_down = (next.1 > curr.1);
                for i in 1..(next.1 - curr.1).abs() + 1 {
                    if go_down {
                        rocks.push((curr.0, curr.1 + i));
                    } else {
                        rocks.push((curr.0, curr.1 - i));
                    }
                }
            } else if curr.1 == next.1 {
                let go_right = (next.0 > curr.0);
                for i in 1..(next.0 - curr.0).abs() + 1 {
                    if go_right {
                        rocks.push((curr.0 + i, curr.1));
                    } else {
                        rocks.push((curr.0 - i, curr.1));
                    }
                }
            }
            curr = next;
        }
    }
    let sand_origin: (i32, i32) = (500, 0);
    let mut sand: Vec<(i32, i32)> = Vec::new();
    visualize(&rocks, &sand_origin, &sand);
    let mut obstacles = rocks.clone();
    loop {
        let new_sand = match step(obstacles.clone(), &sand_origin) {
            Some(s) => s,
            None => break,
        };
        sand.push(new_sand);
        obstacles.push(new_sand);
        visualize(&rocks, &sand_origin, &sand);
        println!("{}", sand.len());
        sleep(Duration::new(0, 10_000_000));
    }
    println!("{}", sand.len());
}

#[allow(unused)]
fn part2(input: &str) {
    // let lines = input.lines();
}
