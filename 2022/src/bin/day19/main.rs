use std::{
    collections::HashMap,
    ops::{Add, AddAssign, SubAssign},
    thread,
};

use regex::Regex;

/// Advent of Code 2022 - Day 19
/// https://adventofcode.com/2022/day/19
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

struct Blueprint {
    id: u32,
    /// ore
    ore_robot_cost: u32,
    /// ore
    clay_robot_cost: u32,
    /// ore, clay
    obsidian_robot_cost: [u32; 2],
    /// ore, obsidian
    geode_robot_cost: [u32; 2],
}

#[derive(Eq, Hash, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Simulation {
    blueprint: Blueprint,
    resources: HashMap<Resource, u32>,
    robots: HashMap<Resource, u32>,
}

impl Simulation {
    fn new(blueprint: Blueprint) -> Self {
        let mut resources = HashMap::new();
        resources.insert(Resource::Ore, 0);
        resources.insert(Resource::Clay, 0);
        resources.insert(Resource::Obsidian, 0);
        resources.insert(Resource::Geode, 0);

        let mut robots = HashMap::new();
        robots.insert(Resource::Ore, 1);
        robots.insert(Resource::Clay, 0);
        robots.insert(Resource::Obsidian, 0);
        robots.insert(Resource::Geode, 0);

        Simulation {
            blueprint,
            resources,
            robots,
        }
    }

    fn produce_resources(&mut self) {
        for (resource, count) in self.resources.iter_mut() {
            *count += self.robots[resource];
        }
    }

    /// returns a vec of bought robots, mutates the resources
    fn spend_resources_optimally(&mut self, minute: u32, total_minutes: u32) -> Option<Resource> {
        let mut robot_to_build = None;
        let ore = self.resources.get_mut(&Resource::Ore).unwrap();
        let clay = self.resources.get_mut(&Resource::Clay).unwrap();
        let obsidian = self.resources.get_mut(&Resource::Obsidian).unwrap();

        if self.blueprint.geode_robot_cost[0] >= *ore
            && self.blueprint.geode_robot_cost[1] >= *obsidian
        {
            ore.sub_assign(self.blueprint.geode_robot_cost[0]);
            obsidian.sub_assign(self.blueprint.geode_robot_cost[1]);
            robot_to_build = Some(Resource::Geode);
        }

        robot_to_build
    }

    fn run_for(&mut self, minutes: u32) {
        for minute in 1..=minutes {
            let robots = self.spend_resources_optimally(minute, minutes);
            self.produce_resources();
            for robot in robots {
                let production = self.robots.get_mut(&robot).unwrap();
                production.add_assign(1);
            }
        }
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut simulations = vec![];

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    for line in input.lines() {
        let (_, values) = re.captures(line).unwrap().extract::<7>();
        let [
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost_ore,
            obsidian_robot_cost_clay,
            geode_robot_cost_ore,
            geode_robot_cost_obsidian,
        ] = values.map(|s| s.parse::<u32>().unwrap());
        let blueprint = Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost: [obsidian_robot_cost_ore, obsidian_robot_cost_clay],
            geode_robot_cost: [geode_robot_cost_ore, geode_robot_cost_obsidian],
        };
        simulations.push(Simulation::new(blueprint));
    }

    let mut handles = vec![];
    for sim in simulations {
        handles.push(thread::spawn(move || {
            let mut sim = sim;
            sim.run_for(24);
            sim
        }));
    }
    let mut final_simulations = vec![];

    for handle in handles {
        match handle.join() {
            Ok(sim_result) => {
                final_simulations.push(sim_result);
            }
            Err(e) => {
                eprintln!("A thread panicked and failed to join: {:?}", e);
            }
        }
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
