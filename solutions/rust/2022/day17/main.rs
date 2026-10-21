use std::{
    collections::{HashMap, HashSet},
    u64,
};

/// Advent of Code 2022 - Day 17
/// https://adventofcode.com/2022/day/17
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[derive(Debug, Clone, PartialEq, Hash)]
enum Shape {
    HorizontalLine,
    Plus,
    L,
    VerticalLine,
    Square,
}

type Coords = (i64, i64);

impl Eq for Shape {}

impl Shape {
    fn next(&self) -> Shape {
        match self {
            Shape::HorizontalLine => Shape::Plus,
            Shape::Plus => Shape::L,
            Shape::L => Shape::VerticalLine,
            Shape::VerticalLine => Shape::Square,
            Shape::Square => Shape::HorizontalLine,
        }
    }

    fn height(&self) -> i64 {
        match self {
            Shape::HorizontalLine => 1,
            Shape::Plus => 3,
            Shape::L => 3,
            Shape::VerticalLine => 4,
            Shape::Square => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    curr_shape: Shape,
    curr_block: Block,
    occupied_spaces: HashSet<Coords>,
    height: i64,
    width: i64,

    spawn_left_padding: i64,
    spawn_bottom_padding: i64,
}

type Block = Vec<Coords>;

impl Board {
    fn new(height: i64, width: i64) -> Self {
        let mut board = Board {
            curr_shape: Shape::HorizontalLine,
            curr_block: vec![],
            occupied_spaces: HashSet::new(),
            height,
            width,
            spawn_left_padding: 2,
            spawn_bottom_padding: 3,
        };
        board.curr_block = board.spawn_shape();
        return board;
    }

    fn spawn_shape(&self) -> Block {
        let spawn_left: i64 = self.spawn_left_padding;
        let spawn_bottom = self.height + self.spawn_bottom_padding + 1;

        match self.curr_shape {
            Shape::HorizontalLine => vec![
                (spawn_left, spawn_bottom),
                (spawn_left + 1, spawn_bottom),
                (spawn_left + 2, spawn_bottom),
                (spawn_left + 3, spawn_bottom),
            ],
            Shape::Plus => vec![
                (spawn_left + 1, spawn_bottom),
                (spawn_left, spawn_bottom + 1),
                (spawn_left + 1, spawn_bottom + 1),
                (spawn_left + 2, spawn_bottom + 1),
                (spawn_left + 1, spawn_bottom + 2),
            ],
            Shape::L => vec![
                (spawn_left, spawn_bottom),
                (spawn_left + 1, spawn_bottom),
                (spawn_left + 2, spawn_bottom),
                (spawn_left + 2, spawn_bottom + 1),
                (spawn_left + 2, spawn_bottom + 2),
            ],
            Shape::VerticalLine => vec![
                (spawn_left, spawn_bottom),
                (spawn_left, spawn_bottom + 1),
                (spawn_left, spawn_bottom + 2),
                (spawn_left, spawn_bottom + 3),
            ],
            Shape::Square => vec![
                (spawn_left, spawn_bottom),
                (spawn_left + 1, spawn_bottom),
                (spawn_left, spawn_bottom + 1),
                (spawn_left + 1, spawn_bottom + 1),
            ],
        }
    }

    fn try_move_down(&self, block: &Block) -> Option<Block> {
        let new_coords = block
            .iter()
            .clone()
            .map(|&(x, y)| (x, y - 1))
            .collect::<Block>();
        for pos in new_coords.iter() {
            if pos.1 == -1 || self.occupied_spaces.contains(&pos) {
                return None;
            }
        }
        Some(new_coords)
    }

    fn try_move_right(&self, block: &Block) -> Option<Block> {
        let new_coords = block
            .iter()
            .clone()
            .map(|&(x, y)| (x + 1, y))
            .collect::<Block>();
        for pos in new_coords.iter() {
            if pos.0 == (self.width) || self.occupied_spaces.contains(&pos) {
                return Some(block.clone());
            }
        }
        Some(new_coords)
    }

    fn try_move_left(&self, block: &Block) -> Option<Block> {
        let new_coords = block
            .iter()
            .clone()
            .map(|&(x, y)| (x - 1, y))
            .collect::<Block>();
        for pos in new_coords.iter() {
            if pos.0 == -1 || self.occupied_spaces.contains(&pos) {
                return Some(block.clone());
            }
        }
        Some(new_coords)
    }

    fn freeze_block(&mut self, block: &Block) {
        for pos in block.iter() {
            self.occupied_spaces.insert(*pos);
        }
        self.height = self
            .height
            .max(block.iter().fold(0, |acc, &(_, y)| y.max(acc)));
    }

    fn get_top_surface_profile(&self) -> Vec<i64> {
        // default to floor if no blocks
        let mut column_heights = vec![-1; self.width as usize];

        for x in 0..self.width {
            let col_max_y = self
                .occupied_spaces
                .iter()
                .filter(|&&(px, _)| px == x)
                .map(|&(_, py)| py)
                .max()
                .unwrap_or(-1); // if no blocks in column, it's at the floor level (y=-1)
            column_heights[x as usize] = col_max_y;
        }

        let min_col_height = *column_heights.iter().min().unwrap_or(&-1);

        column_heights.iter().map(|h| h - min_col_height).collect()
    }

    fn print(self) {
        for y in (0..=(self.height + self.curr_shape.height() + self.spawn_bottom_padding)).rev() {
            print!("|");
            for x in 0..self.width {
                let at_moving_block = self.curr_block.contains(&(x, y));
                let at_frozen_block = self.occupied_spaces.contains(&(x, y));

                if at_moving_block {
                    print!("@")
                } else if at_frozen_block {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            print!("|");
            println!()
        }
        println!("+{}+", "-".repeat(self.width as usize))
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut block_amount = 2022;
    let mut board = Board::new(-1, 7);

    let moves = input.chars().collect::<Vec<char>>();
    loop {
        for _move in &moves {
            if block_amount == 0 {
                board.clone().print();
                println!("{}", board.height + 1);
                return;
            }
            match _move {
                '>' => {
                    let Some(block) = board.try_move_right(&board.curr_block) else {
                        unreachable!()
                    };
                    board.curr_block = block;
                }
                '<' => {
                    let Some(block) = board.try_move_left(&board.curr_block) else {
                        unreachable!()
                    };
                    board.curr_block = block;
                }
                _ => unreachable!(),
            }
            let Some(block) = board.try_move_down(&board.curr_block) else {
                board.freeze_block(&board.curr_block.clone());
                block_amount -= 1;
                board.curr_shape = board.curr_shape.next();
                board.curr_block = board.spawn_shape();
                continue;
            };
            board.curr_block = block;
        }
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let target_amount: u64 = 1_000_000_000_000;
    let mut block_counter: u64 = 0;
    let mut total_height: u64 = 0;

    let moves_chars = input.chars().collect::<Vec<char>>();
    let mut jet_index: usize = 0;

    let mut board = Board::new(-1, 7); // initial height -1 implies floor at y=0

    // Key: (Current_Shape, current_jet_index, top_surface_mask)
    // Value: (blocks_fallen_at_this_state, total_height_at_this_state)
    let mut memo = HashMap::<(Shape, usize, Vec<i64>), (u64, u64)>::new();

    loop {
        if block_counter >= target_amount {
            break;
        }

        let current_state_key = (
            board.curr_shape.clone(),        // Shape of the block about to fall
            jet_index,                       // Next jet to be applied
            board.get_top_surface_profile(), // Shape of the top of the tower
        );

        if let Some(&(prev_block_count, prev_total_height)) = memo.get(&current_state_key) {
            // Cycle detected
            let blocks_in_cycle = block_counter - prev_block_count;
            let height_in_cycle = total_height - prev_total_height;

            let remaining_blocks = target_amount - block_counter;
            let num_cycles_to_skip = remaining_blocks / blocks_in_cycle;

            block_counter += num_cycles_to_skip * blocks_in_cycle;
            total_height += num_cycles_to_skip * height_in_cycle;
            // println!(
            //     "Skipped {} cycles. New block_counter: {}, new total_height: {}",
            //     num_cycles_to_skip, block_counter, total_height
            // );
            break;
        } else {
            memo.insert(current_state_key, (block_counter, total_height));
        }

        board.curr_block = board.spawn_shape();

        loop {
            let current_jet = moves_chars[jet_index];
            let next_block_after_jet = match current_jet {
                '>' => board.try_move_right(&board.curr_block).unwrap(),
                '<' => board.try_move_left(&board.curr_block).unwrap(),
                _ => unreachable!(),
            };
            board.curr_block = next_block_after_jet;

            jet_index = (jet_index + 1) % moves_chars.len();

            // gravity
            if let Some(block_after_down) = board.try_move_down(&board.curr_block) {
                board.curr_block = block_after_down;
            } else {
                board.freeze_block(&board.curr_block.clone());
                block_counter += 1;
                total_height = board.height as u64 + 1; // max_y + 1 since y starts at 0

                board.curr_shape = board.curr_shape.next();
                break;
            }
        }
    }
    // set new board state
    let mut relevant_occupied_spaces = HashSet::new();
    let surface = board
        .get_top_surface_profile()
        .iter()
        .enumerate()
        .map(|(i, y)| (i as i64, *y))
        .collect::<Vec<Coords>>();
    let highest_surface_point: i64 = surface.iter().fold(0, |acc, (_, y)| acc.max(*y));
    let relative_height_offset = (total_height as i64 - 1) - highest_surface_point;
    for &(x, y) in surface.iter() {
        relevant_occupied_spaces.insert((x, relative_height_offset + y));
    }
    board.occupied_spaces = relevant_occupied_spaces;
    board.height = (total_height - 1) as i64;

    while block_counter < target_amount {
        board.curr_block = board.spawn_shape();

        loop {
            let current_jet = moves_chars[jet_index];
            let next_block_after_jet = match current_jet {
                '>' => board.try_move_right(&board.curr_block).unwrap(),
                '<' => board.try_move_left(&board.curr_block).unwrap(),
                _ => unreachable!(),
            };
            board.curr_block = next_block_after_jet;

            jet_index = (jet_index + 1) % moves_chars.len();

            // gravity
            if let Some(block_after_down) = board.try_move_down(&board.curr_block) {
                board.curr_block = block_after_down;
            } else {
                board.freeze_block(&board.curr_block.clone());
                block_counter += 1;
                total_height = board.height as u64 + 1;

                // If target amount is reached after this block, stop.
                if block_counter == target_amount {
                    break;
                }

                board.curr_shape = board.curr_shape.next();
                break;
            }
        }
    }

    println!("{}", board.height.clone() + 1);
}
