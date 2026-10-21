/// Advent of Code 2022 - Day 23
/// https://adventofcode.com/2022/day/23
fn main() {
    let input = include_str!("./test_input.txt");
    part1(&input);
    // part2(&input);
}

type Coords = (i32, i32);

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Elf {
    pos: Coords,
    dest: Option<Coords>,
}

#[allow(unused)]
fn part1(input: &str) {
    let mut elves: Vec<Elf> = vec![];
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            elves.push(Elf {
                pos: (row as i32, col as i32),
                dest: None,
            });
        }
    }
    const ROUNDS: i32 = 10;
    for _ in 0..ROUNDS {
        // phase 1: Proposals
        let cloned_elves = elves.clone();
        for elf in elves.iter_mut() {
            // check N, NE, NW
            let n = (elf.pos.0 - 1, elf.pos.1);
            let ne = (elf.pos.0 - 1, elf.pos.1 + 1);
            let nw = (elf.pos.0 - 1, elf.pos.1 - 1);
            if !cloned_elves
                .iter()
                .any(|e| e.pos == n || e.pos == ne || e.pos == nw)
            {
                elf.dest = Some(n);
                continue;
            }
            // check S, SE, SW
            let s = (elf.pos.0 + 1, elf.pos.1);
            let se = (elf.pos.0 + 1, elf.pos.1 + 1);
            let sw = (elf.pos.0 + 1, elf.pos.1 - 1);
            if !cloned_elves
                .iter()
                .any(|e| e.pos == s || e.pos == se || e.pos == sw)
            {
                elf.dest = Some(s);
                continue;
            }
            // check W, NW, SW
            let w = (elf.pos.0, elf.pos.1 - 1);
            if !cloned_elves
                .iter()
                .any(|e| e.pos == w || e.pos == nw || e.pos == sw)
            {
                elf.dest = Some(w);
                continue;
            }
            // check E, NE, SE
            let e = (elf.pos.0, elf.pos.1 + 1);
            if !cloned_elves
                .iter()
                .any(|el| el.pos == e || el.pos == ne || el.pos == se)
            {
                elf.dest = Some(e);
                continue;
            }
        }
        let elves_with_dest = elves
            .clone()
            .into_iter()
            .filter(|&elf| elf.dest.is_some())
            .collect::<Vec<Elf>>();
        // phase 2: Movement
        for elf in elves.iter_mut() {
            match elf.dest {
                None => continue,
                Some(dest) => {
                    if !elves_with_dest
                        .iter()
                        .any(|&ce| ce.dest.unwrap() == dest && ce.pos != elf.pos)
                    {
                        // println!(
                        //     "Elf {},{} moving to {},{}",
                        //     elf.pos.0, elf.pos.1, dest.0, dest.1
                        // );
                        elf.pos = dest;
                    }
                }
            }
        }
    }

    visualize(&elves);
    println!("{}", count_ground(&elves))
}

fn count_ground(elves: &Vec<Elf>) -> i32 {
    let (min_row, max_row) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), elf| {
        let new_min = std::cmp::min(min, elf.pos.0);
        let new_max = std::cmp::max(max, elf.pos.0);
        (new_min, new_max)
    });
    let (min_col, max_col) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), elf| {
        let new_min = std::cmp::min(min, elf.pos.1);
        let new_max = std::cmp::max(max, elf.pos.1);
        (new_min, new_max)
    });
    let width = max_col - min_col;
    let height = max_row - min_row;
    (width * height) - elves.len() as i32
}

fn visualize(elves: &Vec<Elf>) {
    let (min_row, max_row) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), elf| {
        let new_min = std::cmp::min(min, elf.pos.0);
        let new_max = std::cmp::max(max, elf.pos.0);
        (new_min, new_max)
    });
    let (min_col, max_col) = elves.iter().fold((i32::MAX, i32::MIN), |(min, max), elf| {
        let new_min = std::cmp::min(min, elf.pos.1);
        let new_max = std::cmp::max(max, elf.pos.1);
        (new_min, new_max)
    });
    let width = max_col - min_col;
    let height = max_row - min_row;
    let mut grid = vec![vec!['.'; width as usize + 1]; height as usize + 1];
    for elf in elves {
        let row = elf.pos.0 - min_row;
        let col = elf.pos.1 - min_col;
        grid[row as usize][col as usize] = '#';
    }
    for row in 0..height {
        for col in 0..width {
            print!("{}", grid[row as usize][col as usize]);
        }
        println!();
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
