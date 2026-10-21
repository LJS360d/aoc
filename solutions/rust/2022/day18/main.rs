use std::collections::HashSet;
use std::collections::VecDeque;

/// Advent of Code 2022 - Day 18
/// https://adventofcode.com/2022/day/18
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

type Pos = (i32, i32, i32);

fn valid_neighbors(block: &Pos) -> [Pos; 6] {
    let &(x, y, z) = block;
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
}

fn find_neighbors(block: &Pos, all_blocks: &Vec<Pos>) -> Vec<Pos> {
    let mut neighbors = vec![];
    let valid_neighbors = valid_neighbors(block);
    for neighbor in valid_neighbors {
        if all_blocks.contains(&neighbor) {
            neighbors.push(neighbor);
        }
    }
    neighbors
}

fn identify_neighbors(block: &Pos, all_blocks: &HashSet<Pos>) -> (Vec<Pos>, Vec<Pos>) {
    let mut block_neighbors = vec![];
    let mut air_neighbors = vec![];
    let valid_neighbors = valid_neighbors(block);
    for neighbor in valid_neighbors {
        if all_blocks.contains(&neighbor) {
            block_neighbors.push(neighbor);
        } else {
            air_neighbors.push(neighbor);
        }
    }
    (block_neighbors, air_neighbors)
}

// BFS
fn is_free_air(
    start_air_block: &Pos,
    lava_blocks: &HashSet<Pos>,
    min_coord: &Pos,
    max_coord: &Pos,
) -> bool {
    let (min_x, min_y, min_z) = *min_coord;
    let (max_x, max_y, max_z) = *max_coord;

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut queue: VecDeque<Pos> = VecDeque::new();

    queue.push_back(*start_air_block);
    visited.insert(*start_air_block);

    while let Some(current_air_block) = queue.pop_front() {
        let (x, y, z) = current_air_block;

        // Check if we reached outside the bounding box (with a buffer of 1)
        if x < min_x - 1
            || x > max_x + 1
            || y < min_y - 1
            || y > max_y + 1
            || z < min_z - 1
            || z > max_z + 1
        {
            return true;
        }

        let valid_neighbors = valid_neighbors(&current_air_block);

        for neighbor in valid_neighbors {
            // If it's not a lava block and not visited, add to queue
            if !lava_blocks.contains(&neighbor) && visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // All reachable air blocks are within the bounding box and not connected to free air
    false
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut all_blocks: Vec<Pos> = vec![];
    for line in lines {
        let vec_str = line.split(",").collect::<Vec<&str>>();
        let x = vec_str[0].parse::<i32>().unwrap();
        let z = vec_str[2].parse::<i32>().unwrap();
        let y = vec_str[1].parse::<i32>().unwrap();
        all_blocks.push((x, y, z));
    }
    let mut area = 0;
    for block in all_blocks.iter() {
        let neighbors = find_neighbors(&block, &all_blocks);
        area += (6 - neighbors.len())
    }
    println!("{}", area);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let mut all_blocks_vec: Vec<Pos> = vec![];
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_z = i32::MAX;
    let mut max_z = i32::MIN;

    for line in lines {
        let vec_str = line.split(",").collect::<Vec<&str>>();
        let x = vec_str[0].parse::<i32>().unwrap();
        let y = vec_str[1].parse::<i32>().unwrap();
        let z = vec_str[2].parse::<i32>().unwrap();

        all_blocks_vec.push((x, y, z));

        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        min_z = min_z.min(z);
        max_z = max_z.max(z);
    }

    let all_blocks_set: HashSet<Pos> = all_blocks_vec.iter().cloned().collect();
    let min_coord = (min_x, min_y, min_z);
    let max_coord = (max_x, max_y, max_z);

    let mut area = 0;
    let mut air_pockets_faces = 0;
    let mut air_pockets_set: HashSet<Pos> = HashSet::new();

    for block in all_blocks_vec.iter() {
        let (block_neighbors, air_neighbors) = identify_neighbors(&block, &all_blocks_set);
        area += (6 - block_neighbors.len());

        for air_block in air_neighbors {
            // Only start a new BFS if this air_block hasn't been identified as part of an air pocket yet
            if !air_pockets_set.contains(&air_block)
                && !is_free_air(&air_block, &all_blocks_set, &min_coord, &max_coord)
            {
                // This air_block is part of a new, undiscovered air pocket.
                // Perform a BFS to find all connected air blocks that are also part of this pocket.
                let mut pocket_queue: VecDeque<Pos> = VecDeque::new();

                pocket_queue.push_back(air_block);
                // mark as visited
                air_pockets_set.insert(air_block);

                while let Some(current_air_pocket_block) = pocket_queue.pop_front() {
                    let pocket_neighbors = valid_neighbors(&current_air_pocket_block);
                    for pn in pocket_neighbors {
                        if !all_blocks_set.contains(&pn) && air_pockets_set.insert(pn) {
                            pocket_queue.push_back(pn);
                        }
                    }
                }
            }
        }
    }

    // Now calculate the surface area contributed by air pockets
    for pocket_block in air_pockets_set.iter() {
        let neighbors = valid_neighbors(pocket_block);
        for neighbor in neighbors {
            if all_blocks_set.contains(&neighbor) {
                air_pockets_faces += 1;
            }
        }
    }

    println!("{}", area - air_pockets_faces);
}
