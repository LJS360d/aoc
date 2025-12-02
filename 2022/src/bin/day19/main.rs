use std::{collections::HashMap, ops::AddAssign, thread};

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

#[derive(Eq, Hash, PartialEq, Debug)]
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
        println!("producing {:?}", self.robots);
        for (resource, count) in self.resources.iter_mut() {
            *count += self.robots[resource];
        }
        println!("resources {:?}", self.resources)
    }

    /// returns a vec of bought robots, mutates the resources
    fn spend_resources_optimally(&mut self, minute: u32, total_minutes: u32) -> Option<Resource> {
        let mut robot_to_build = None;

        let current_ore = *self.resources.get(&Resource::Ore).unwrap();
        let current_clay = *self.resources.get(&Resource::Clay).unwrap();
        let current_obsidian = *self.resources.get(&Resource::Obsidian).unwrap();

        // Prioritize building geode robots
        if current_ore >= self.blueprint.geode_robot_cost[0]
            && current_obsidian >= self.blueprint.geode_robot_cost[1]
        {
            *self.resources.get_mut(&Resource::Ore).unwrap() -= self.blueprint.geode_robot_cost[0];
            *self.resources.get_mut(&Resource::Obsidian).unwrap() -=
                self.blueprint.geode_robot_cost[1];
            robot_to_build = Some(Resource::Geode);
        } else if current_ore >= self.blueprint.obsidian_robot_cost[0]
            && current_clay >= self.blueprint.obsidian_robot_cost[1]
        {
            *self.resources.get_mut(&Resource::Ore).unwrap() -=
                self.blueprint.obsidian_robot_cost[0];
            *self.resources.get_mut(&Resource::Clay).unwrap() -=
                self.blueprint.obsidian_robot_cost[1];
            robot_to_build = Some(Resource::Obsidian);
        } else if current_ore >= self.blueprint.clay_robot_cost
            && (current_clay < self.blueprint.obsidian_robot_cost[1])
        {
            *self.resources.get_mut(&Resource::Ore).unwrap() -= self.blueprint.clay_robot_cost;
            robot_to_build = Some(Resource::Clay);
        } else if current_ore >= self.blueprint.ore_robot_cost
            && (current_ore < self.blueprint.clay_robot_cost
                && current_ore < self.blueprint.obsidian_robot_cost[0]
                && current_ore < self.blueprint.geode_robot_cost[0])
        {
            *self.resources.get_mut(&Resource::Ore).unwrap() -= self.blueprint.ore_robot_cost;
            robot_to_build = Some(Resource::Ore);
        }

        robot_to_build
    }

    fn run_for(&mut self, minutes: u32) {
        for minute in 1..=minutes {
            println!("Blueprint {}: Minute {minute}", self.blueprint.id);
            let robot = self.spend_resources_optimally(minute, minutes);
            self.produce_resources();
            match robot {
                Some(resource) => self.robots.get_mut(&resource).unwrap().add_assign(1),
                None => {}
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

    // Multi-threaded version
    // let mut handles = vec![];
    // for sim in simulations {
    //     handles.push(thread::spawn(move || {
    //         let mut sim = sim;
    //         sim.run_for(24);
    //         sim
    //     }));
    // }
    // let mut final_simulations = vec![];

    // for handle in handles {
    //     match handle.join() {
    //         Ok(sim_result) => {
    //             final_simulations.push(sim_result);
    //         }
    //         Err(e) => {
    //             eprintln!("A thread panicked and failed to join: {:?}", e);
    //         }
    //     }
    // }

    // Sync version
    for sim in simulations.iter_mut() {
        sim.run_for(24);
        println!("");
        println!("---------------");
        println!("");
    }

    println!(
        "{}",
        simulations.iter().fold(0, |acc, sim| {
            return acc + (sim.blueprint.id * sim.resources.get(&Resource::Geode).unwrap());
        })
    )
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
