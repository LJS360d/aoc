/// Advent of Code 2022 - Day 14
/// https://adventofcode.com/2022/day/14
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

fn visualize(rocks: &Vec<Coords>, sand_origin: &Coords, sand: &Vec<Coords>) {
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
    let padding = 2;
    for y in -padding..height + padding {
        for x in -padding..width + padding {
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

type Coords = (i32, i32);

fn step(obstacles: &Vec<Coords>, start_pos: &Coords) -> Option<Coords> {
    let next_obs = get_next_obstacle(&obstacles, start_pos);
    if next_obs.is_none() {
        return Some(*start_pos);
    }
    let next_obs = next_obs.unwrap();
    if next_obs.1 == -1 {
        return None;
    }

    // left
    let try_pos: Option<Coords> = Some((next_obs.0 - 1, next_obs.1));
    let left_obs = get_next_obstacle(&obstacles, &try_pos.unwrap());
    match left_obs {
        None => {
            // sand cannot be placed here
        }
        Some(pos) => {
            if pos.1 == -1 {
                return None;
            }
            let left_pos = (pos.0, pos.1 - 1);
            return step(&obstacles, &left_pos);
        }
    }

    // right
    let try_pos = Some((next_obs.0 + 1, next_obs.1));
    let right_obs = get_next_obstacle(&obstacles, &try_pos.unwrap());
    match right_obs {
        None => {
            // sand cannot be placed here
        }
        Some(pos) => {
            if pos.1 == -1 {
                return None;
            }
            let right_pos = (pos.0, pos.1 - 1);
            return step(&obstacles, &right_pos);
        }
    }

    return Some((next_obs.0, next_obs.1 - 1));
}

fn get_next_obstacle(obstacles: &Vec<Coords>, start_pos: &Coords) -> Option<Coords> {
    let mut next_obs: Option<Coords> = None;
    for obs in obstacles
        .iter()
        .filter(|(x, y)| *x == start_pos.0 && *y >= start_pos.1)
    {
        if obs.1 == start_pos.1 {
            // the highest obstacle is directly on the start position
            // sand cannot be placed there
            return None;
        }
        if next_obs.is_none() || next_obs.unwrap().1 > obs.1 {
            next_obs = Some(*obs);
        }
    }
    let next_obs = match next_obs {
        Some(n) => n,
        // there are no obstacles under start_pos, so it falls into the void
        None => return Some((-1, -1)),
    };

    // i now posses the highest obstacle that can be reached by the sand
    // if next_obs.1 == (start_pos.1 - 1) {
    //     // if its directly under the start pos, then the sand is blocked
    //     return None;
    // }
    return Some(next_obs);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut rocks: Vec<Coords> = vec![];
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
    let sand_origin: Coords = (500, 0);
    let mut sand: Vec<Coords> = Vec::new();
    // visualize(&rocks, &sand_origin, &sand);
    let mut obstacles = rocks.clone();
    loop {
        let new_sand = step(&obstacles, &sand_origin);
        if new_sand.is_none() {
            break;
        }
        let new_sand = new_sand.unwrap();
        let latest_sand = sand.last();
        match latest_sand {
            None => {}
            Some(s) => {
                if s.1 == new_sand.1 && s.0 == new_sand.0 {
                    break;
                }
            }
        }
        sand.push(new_sand);
        obstacles.push(new_sand);
    }
    visualize(&rocks, &sand_origin, &sand);
    println!("{}", sand.len());
}

#[allow(unused)]
fn part2(input: &str) {
    // let lines = input.lines();
}
