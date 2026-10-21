use std::collections::HashSet;

/// Advent of Code 2022 - Day 9
/// https://adventofcode.com/2022/day/9
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    let mut visited = HashSet::<[i32; 2]>::new();
    visited.insert(tail.clone());

    fn update_tail(tail: &mut [i32; 2], head: [i32; 2], visited: &mut HashSet<[i32; 2]>) {
        let valid_relative_positions: [[i32; 2]; 9] = [
            [0, 0],
            [0, 1],
            [1, 0],
            [1, 1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [1, -1],
            [-1, -1],
        ];
        for [rel_x, rel_y] in valid_relative_positions {
            let valid_pos = [head[0] + rel_x, head[1] + rel_y];
            if *tail == valid_pos {
                // tail is already in a valid position, no need to update
                return;
            }
        }

        /* diagonal movements */
        if (tail[0] < head[0] && tail[1] < head[1]) {
            // tail is bottom left of head 1 apart -> move 1 up right
            tail[0] += 1;
            tail[1] += 1;
        } else if tail[0] > head[0] && tail[1] > head[1] {
            // tail is up right of head 1 apart -> move 1 down left
            tail[0] -= 1;
            tail[1] -= 1;
        } else if tail[0] < head[0] && tail[1] > head[1] {
            // tail is bottom right of head 1 apart -> move 1 up left
            tail[0] += 1;
            tail[1] -= 1;
        } else if tail[0] > head[0] && tail[1] < head[1] {
            // tail is up left of head 1 apart -> move 1 down right
            tail[0] -= 1;
            tail[1] += 1;
        }
        /* straight movements */
        else if tail[0] == head[0] - 2 && tail[1] == head[1] {
            // tail is left -> move 1 right
            tail[0] += 1;
        } else if tail[0] == head[0] + 2 && tail[1] == head[1] {
            // tail is right -> move 1 left
            tail[0] -= 1;
        } else if tail[1] == head[1] - 2 && tail[0] == head[0] {
            // tail is up -> move 1 down
            tail[1] += 1;
        } else if tail[1] == head[1] + 2 && tail[0] == head[0] {
            // tail is down -> move 1 up
            tail[1] -= 1;
        }
        visited.insert(tail.clone());
    }

    for line in lines {
        let (direction, steps_str) = line.split_once(" ").unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "L" => {
                    head[0] -= 1;
                    update_tail(&mut tail, head, &mut visited);
                }
                "R" => {
                    head[0] += 1;
                    update_tail(&mut tail, head, &mut visited);
                }
                "U" => {
                    head[1] += 1;
                    update_tail(&mut tail, head, &mut visited);
                }
                "D" => {
                    head[1] -= 1;
                    update_tail(&mut tail, head, &mut visited);
                }
                _ => {}
            }
        }
    }

    println!("{:?}", visited.len());
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut chain: [[i32; 2]; 10] = [
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
        [0, 0],
    ];
    let mut visited = HashSet::<[i32; 2]>::new();
    visited.insert([0, 0]);

    fn update_knot(tail: &mut [i32; 2], head: [i32; 2]) {
        let valid_relative_positions: [[i32; 2]; 9] = [
            [0, 0],
            [0, 1],
            [1, 0],
            [1, 1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [1, -1],
            [-1, -1],
        ];
        for [rel_x, rel_y] in valid_relative_positions {
            let valid_pos = [head[0] + rel_x, head[1] + rel_y];
            if *tail == valid_pos {
                // tail is already in a valid position, no need to update
                return;
            }
        }

        /* diagonal movements */
        if (tail[0] < head[0] && tail[1] < head[1]) {
            // tail is bottom left of head 1 apart -> move 1 up right
            tail[0] += 1;
            tail[1] += 1;
        } else if tail[0] > head[0] && tail[1] > head[1] {
            // tail is up right of head 1 apart -> move 1 down left
            tail[0] -= 1;
            tail[1] -= 1;
        } else if tail[0] < head[0] && tail[1] > head[1] {
            // tail is bottom right of head 1 apart -> move 1 up left
            tail[0] += 1;
            tail[1] -= 1;
        } else if tail[0] > head[0] && tail[1] < head[1] {
            // tail is up left of head 1 apart -> move 1 down right
            tail[0] -= 1;
            tail[1] += 1;
        }
        /* straight movements */
        else if tail[0] == head[0] - 2 && tail[1] == head[1] {
            // tail is left -> move 1 right
            tail[0] += 1;
        } else if tail[0] == head[0] + 2 && tail[1] == head[1] {
            // tail is right -> move 1 left
            tail[0] -= 1;
        } else if tail[1] == head[1] - 2 && tail[0] == head[0] {
            // tail is up -> move 1 down
            tail[1] += 1;
        } else if tail[1] == head[1] + 2 && tail[0] == head[0] {
            // tail is down -> move 1 up
            tail[1] -= 1;
        }
    }

    for line in lines {
        let (direction, steps_str) = line.split_once(" ").unwrap();
        let steps = steps_str.parse::<i32>().unwrap();

        for _ in 0..steps {
            match direction {
                "L" => {
                    chain[0][0] -= 1;
                }
                "R" => {
                    chain[0][0] += 1;
                }
                "U" => {
                    chain[0][1] += 1;
                }
                "D" => {
                    chain[0][1] -= 1;
                }
                _ => {}
            }
            for i in 1..chain.len() {
                let prev = chain[i - 1];
                let mut curr = chain[i];
                update_knot(&mut curr, prev);
                chain[i] = curr;
            }
            visited.insert(chain[chain.len() - 1].clone());

        }
    }

    println!("{:?}", visited.len());
}
