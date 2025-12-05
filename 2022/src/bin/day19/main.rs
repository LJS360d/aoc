use std::{cmp::max, collections::HashMap};

use regex::Regex;

/// Advent of Code 2022 - Day 19
/// https://adventofcode.com/2022/day/19
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_costs: [u32; 2], // [Ore, Clay]
    geode_robot_costs: [u32; 2],    // [Ore, Obsidian]

    max_ore_needed_per_turn: u32,
    max_clay_needed_per_turn: u32,
    max_obsidian_needed_per_turn: u32,
}

const ORE_INDEX: usize = 0;
const CLAY_INDEX: usize = 1;
const OBSIDIAN_INDEX: usize = 2;
const GEODE_INDEX: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchState {
    current_resources: [u32; 4], // [Ore, Clay, Obsidian, Geode]
    robot_inventory: [u32; 4], // [Ore_Bot_Count, Clay_Bot_Count, Obsidian_Bot_Count, Geode_Bot_Count]
}
// Memoization cache: (Time Remaining, State) -> Max Geodes
type StateCache = HashMap<(u32, SearchState), u32>;

fn find_max_geodes(
    time_remaining: u32,
    current_state: SearchState,
    blueprint: &Blueprint,
    memo: &mut StateCache,
) -> u32 {
    if time_remaining == 0 {
        return current_state.current_resources[GEODE_INDEX];
    }

    if let Some(&cached_max_geodes) = memo.get(&(time_remaining, current_state)) {
        return cached_max_geodes;
    }

    let mut maximum_geodes_found = current_state.current_resources[GEODE_INDEX]
        + current_state.robot_inventory[GEODE_INDEX] * time_remaining;

    // Pruning: The maximum possible geodes even with optimal building (upper bound)
    // If the maximum potential is less than our current best, we can skip.
    // NOTE: This check is often relative to a global max across all branches,
    // but here we rely on the state-space reduction from resource caps.

    // 1. Attempt to build a GEODE Robot (Highest Priority)
    if current_state.current_resources[ORE_INDEX] >= blueprint.geode_robot_costs[0]
        && current_state.current_resources[OBSIDIAN_INDEX] >= blueprint.geode_robot_costs[1]
    {
        let mut next_state = current_state;

        // Spend resources
        next_state.current_resources[ORE_INDEX] -= blueprint.geode_robot_costs[0];
        next_state.current_resources[OBSIDIAN_INDEX] -= blueprint.geode_robot_costs[1];

        // Resources produced in the minute of building
        for i in 0..4 {
            next_state.current_resources[i] += current_state.robot_inventory[i];
        }

        // Add the newly built robot
        next_state.robot_inventory[GEODE_INDEX] += 1;

        maximum_geodes_found = max(
            maximum_geodes_found,
            find_max_geodes(time_remaining - 1, next_state, blueprint, memo),
        );
    } else {
        // 2. Iterate through other non-geode build options and the 'Wait' option

        // [Ore_Cost, Other_Cost, Robot_Index, Max_Cap]
        let robot_build_options = [
            (
                blueprint.ore_robot_ore_cost,
                0,
                ORE_INDEX,
                blueprint.max_ore_needed_per_turn,
            ),
            (
                blueprint.clay_robot_ore_cost,
                0,
                CLAY_INDEX,
                blueprint.max_clay_needed_per_turn,
            ),
            (
                blueprint.obsidian_robot_costs[0],
                blueprint.obsidian_robot_costs[1],
                OBSIDIAN_INDEX,
                blueprint.max_obsidian_needed_per_turn,
            ),
        ];

        let mut can_afford_any_robot = false;

        for &(ore_cost, other_cost, robot_type_index, max_cap) in robot_build_options.iter() {
            // Pruning: Do not build if we have reached the production cap for this resource
            if current_state.robot_inventory[robot_type_index] >= max_cap {
                continue;
            }

            let other_resource_condition = match robot_type_index {
                OBSIDIAN_INDEX => current_state.current_resources[CLAY_INDEX] >= other_cost,
                _ => true,
            };

            // Check affordability
            if current_state.current_resources[ORE_INDEX] >= ore_cost && other_resource_condition {
                can_afford_any_robot = true;

                let mut next_state = current_state;

                // Spend resources
                next_state.current_resources[ORE_INDEX] -= ore_cost;
                if robot_type_index == OBSIDIAN_INDEX {
                    next_state.current_resources[CLAY_INDEX] -= other_cost;
                }

                // Resources produced in the minute of building
                for i in 0..4 {
                    next_state.current_resources[i] += current_state.robot_inventory[i];
                }

                // Add the newly built robot
                next_state.robot_inventory[robot_type_index] += 1;

                maximum_geodes_found = max(
                    maximum_geodes_found,
                    find_max_geodes(time_remaining - 1, next_state, blueprint, memo),
                );
            }
        }

        // 3. Option to WAIT and gather resources (crucial for finding optimal path)

        // We only prune the 'Wait' action if:
        // 1. We are not saving up for the Geode robot (i.e., we have enough clay/obsidian robots).
        // 2. Our current resources exceed the max cost for any robot that uses that resource.

        let should_explore_wait = current_state.robot_inventory[OBSIDIAN_INDEX]
            < blueprint.max_obsidian_needed_per_turn
            || current_state.robot_inventory[CLAY_INDEX] < blueprint.max_clay_needed_per_turn
            || current_state.robot_inventory[ORE_INDEX] < blueprint.max_ore_needed_per_turn;

        if should_explore_wait || !can_afford_any_robot {
            let mut next_state = current_state;
            // Collect resources for 1 minute
            for i in 0..4 {
                next_state.current_resources[i] += current_state.robot_inventory[i];
            }
            // No robot built
            maximum_geodes_found = max(
                maximum_geodes_found,
                find_max_geodes(time_remaining - 1, next_state, blueprint, memo),
            );
        }
    }

    memo.insert((time_remaining, current_state), maximum_geodes_found);
    maximum_geodes_found
}

#[allow(unused)]
fn part1(input: &str) {
    let mut blueprints = vec![];

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    for line in input.lines() {
        let (_, values) = re.captures(line).unwrap().extract::<7>();
        let [
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        ] = values.map(|s| s.parse::<u32>().unwrap());

        let max_ore_needed_per_turn = *[
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            geode_robot_ore_cost,
        ]
        .iter()
        .max()
        .unwrap();

        let blueprint = Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_costs: [obsidian_robot_ore_cost, obsidian_robot_clay_cost],
            geode_robot_costs: [geode_robot_ore_cost, geode_robot_obsidian_cost],

            // Max production rate needed to afford any robot next turn
            max_ore_needed_per_turn,
            max_clay_needed_per_turn: obsidian_robot_clay_cost,
            max_obsidian_needed_per_turn: geode_robot_obsidian_cost,
        };
        blueprints.push(blueprint);
    }

    let mut acc = 0;

    let init_state = SearchState {
        current_resources: [0, 0, 0, 0],
        robot_inventory: [1, 0, 0, 0],
    };

    for blueprint in blueprints {
        let mut memo = HashMap::new();
        let max_geodes = find_max_geodes(24, init_state, &blueprint, &mut memo);

        let quality_level = blueprint.id * max_geodes;
        acc += quality_level;

        println!(
            "Blueprint {}: Max Geodes = {}, Quality Level = {}",
            blueprint.id, max_geodes, quality_level
        );
    }

    println!("{}", acc);
}

#[allow(unused)]
fn part2(input: &str) {
    let mut blueprints = vec![];

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    for line in input.lines() {
        let (_, values) = re.captures(line).unwrap().extract::<7>();
        let [
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        ] = values.map(|s| s.parse::<u32>().unwrap());

        let max_ore_needed_per_turn = *[
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            geode_robot_ore_cost,
        ]
        .iter()
        .max()
        .unwrap();

        let blueprint = Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_costs: [obsidian_robot_ore_cost, obsidian_robot_clay_cost],
            geode_robot_costs: [geode_robot_ore_cost, geode_robot_obsidian_cost],

            // Max production rate needed to afford any robot next turn
            max_ore_needed_per_turn,
            max_clay_needed_per_turn: obsidian_robot_clay_cost,
            max_obsidian_needed_per_turn: geode_robot_obsidian_cost,
        };
        blueprints.push(blueprint);
        if (blueprints.len() == 3) {
            break;
        }
    }

    let mut acc = 1;

    let init_state = SearchState {
        current_resources: [0, 0, 0, 0],
        robot_inventory: [1, 0, 0, 0],
    };

    for blueprint in blueprints {
        let mut memo = HashMap::new();
        let max_geodes = find_max_geodes(32, init_state, &blueprint, &mut memo);

        acc *= max_geodes;

        println!(
            "Blueprint {}: Max Geodes = {}",
            blueprint.id, max_geodes
        );
    }

    println!("{}", acc);
}
