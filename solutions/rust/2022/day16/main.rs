use regex::Regex;
use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    u32,
};

/// Advent of Code 2022 - Day 16
/// https://adventofcode.com/2022/day/16
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

// (Current Valve Index, Minutes Left, Open Valve Bitmask)
type State = (usize, u32, u32);

struct PipeSystem {
    valves: Vec<Valve>,
    tunnels: Vec<Tunnel>,
    /// A vector of only the 'relevant' valves (AA + flow > 0).
    relevant_valves: Vec<Valve>,
    /// A map of shortest distances between all pairs of relevant valves.
    distances: HashMap<(String, String), u32>,
    memo: HashMap<State, u32>,
}

type OptimalMasks = HashMap<u32, u32>;
impl PipeSystem {
    fn new(valves: Vec<Valve>, tunnels: Vec<Tunnel>) -> Self {
        let mut pipe_system = Self {
            valves,
            tunnels,
            relevant_valves: Vec::new(),
            distances: HashMap::new(),
            memo: HashMap::new(),
        };
        pipe_system.relevant_valves = pipe_system
            .valves
            .iter()
            .filter(|v| v.flow_rate > 0 || v.id == "AA")
            .cloned()
            .collect();

        let relevant_ids: Vec<String> = pipe_system
            .relevant_valves
            .iter()
            .map(|v| v.id.clone())
            .collect();

        let mut distances = HashMap::new();
        for i in &relevant_ids {
            for j in &relevant_ids {
                let dist = pipe_system.shortest_path(i.clone(), j.clone());
                distances.insert((i.clone(), j.clone()), dist);
            }
        }
        pipe_system.distances = distances;
        pipe_system
    }

    fn shortest_path(&self, start: String, end: String) -> u32 {
        if start == end {
            return 0;
        }

        let mut queue = VecDeque::new();
        // (Current Valve ID, Distance)
        queue.push_back((start.to_string(), 0));
        let mut visited = HashMap::new();
        visited.insert(start.to_string(), 0);

        while let Some((current_id, dist)) = queue.pop_front() {
            if current_id == end {
                return dist;
            }

            let neighbors = self.tunnels.iter().filter(|t| t.from == current_id);
            for neighbor in neighbors {
                if !visited.contains_key(&neighbor.to) {
                    visited.insert(neighbor.to.clone(), dist + 1);
                    queue.push_back((neighbor.to.clone(), dist + 1));
                }
            }
        }
        u32::MAX
    }

    pub fn find_max_pressure(
        &mut self,
        current_valve_idx: usize,
        minutes_left: u32,
        open_mask: u32,
    ) -> u32 {
        if minutes_left == 0 {
            return 0;
        }
        let state: State = (current_valve_idx, minutes_left, open_mask);
        if let Some(&pressure) = self.memo.get(&state) {
            return pressure;
        }
        let mut max_total_pressure = 0;
        let current_valve_id = &self.relevant_valves[current_valve_idx].id.clone();
        for (
            next_valve_idx,
            Valve {
                id: next_valve_id,
                flow_rate,
            },
        ) in self.relevant_valves.clone().iter().enumerate()
        {
            let bit = 1 << next_valve_idx;
            if (open_mask & bit) != 0 || *flow_rate == 0 {
                continue;
            }

            let travel_key = (current_valve_id.clone(), next_valve_id.clone());
            let travel_time = *self.distances.get(&travel_key).unwrap();

            let time_to_open = travel_time + 1;

            if time_to_open < minutes_left {
                let new_minutes_left = minutes_left - time_to_open;

                // Pressure released by opening THIS valve from now until the end (30 min)
                let pressure_from_this_valve = flow_rate * new_minutes_left;

                let new_open_mask = open_mask | bit;
                let future_pressure =
                    self.find_max_pressure(next_valve_idx, new_minutes_left, new_open_mask);

                max_total_pressure = max(
                    max_total_pressure,
                    pressure_from_this_valve + future_pressure,
                );
            }
        }

        self.memo.insert(state, max_total_pressure);
        max_total_pressure
    }

    pub fn find_max_pressure_2(
        &mut self,
        current_valve_idx: usize,
        minutes_left: u32,
        open_mask: u32,
        current_pressure: u32,
        max_pressure_for_mask: &mut OptimalMasks,
    ) {
        max_pressure_for_mask
            .entry(open_mask)
            .and_modify(|e| *e = max(*e, current_pressure))
            .or_insert(current_pressure);

        let current_valve_id = self.relevant_valves[current_valve_idx].id.clone();

        for (
            next_valve_idx,
            Valve {
                id: next_valve_id,
                flow_rate,
            },
        ) in self.relevant_valves.clone().iter().enumerate()
        {
            let bit = 1 << next_valve_idx;
            if (open_mask & bit) != 0 || *flow_rate == 0 {
                continue;
            }

            let travel_key = (current_valve_id.clone(), next_valve_id.clone());
            let travel_time = *self.distances.get(&travel_key).unwrap();

            let time_to_open = travel_time + 1;

            if time_to_open < minutes_left {
                let new_minutes_left = minutes_left - time_to_open;

                let pressure_from_this_valve = flow_rate * new_minutes_left;

                let new_open_mask = open_mask | bit;
                self.find_max_pressure_2(
                    next_valve_idx,
                    new_minutes_left,
                    new_open_mask,
                    current_pressure + pressure_from_this_valve,
                    max_pressure_for_mask,
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
}
#[derive(Debug, Clone)]
struct Tunnel {
    // its an edge
    from: String,
    to: String,
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut valves = Vec::new();
    let mut tunnels = Vec::new();

    let re =
        Regex::new(r"Valve (.+) has flow rate=(-?\d+); tunnels? leads? to valves? (.+)").unwrap();
    for line in lines {
        let (_, [name, flow_rate_str, leads_to]) = re.captures(line).unwrap().extract();
        let flow_rate = flow_rate_str.parse::<u32>().unwrap();
        let leads_to = leads_to
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let valve = Valve {
            id: name.to_string(),
            flow_rate,
        };
        valves.push(valve);

        for lead in leads_to {
            let tunnel = Tunnel {
                from: name.to_string(),
                to: lead,
            };
            tunnels.push(tunnel);
        }
    }
    let mut pipe_system = PipeSystem::new(valves, tunnels);
    let curr_valve_index = pipe_system
        .relevant_valves
        .iter()
        .position(|v| v.id == "AA")
        .unwrap();
    let total_pressure = pipe_system.find_max_pressure(curr_valve_index, 30, 0);

    println!("{}", total_pressure);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut valves = Vec::new();
    let mut tunnels = Vec::new();

    let re =
        Regex::new(r"Valve (.+) has flow rate=(-?\d+); tunnels? leads? to valves? (.+)").unwrap();
    for line in lines {
        let (_, [name, flow_rate_str, leads_to]) = re.captures(line).unwrap().extract();
        let flow_rate = flow_rate_str.parse::<u32>().unwrap();
        let leads_to = leads_to
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let valve = Valve {
            id: name.to_string(),
            flow_rate,
        };
        valves.push(valve);

        for lead in leads_to {
            let tunnel = Tunnel {
                from: name.to_string(),
                to: lead,
            };
            tunnels.push(tunnel);
        }
    }
    let mut pipe_system = PipeSystem::new(valves, tunnels);
    let start_valve_index = pipe_system
        .relevant_valves
        .iter()
        .position(|v| v.id == "AA")
        .unwrap();
    let mut max_pressure_for_mask = OptimalMasks::new();

    pipe_system.memo.clear();
    pipe_system.find_max_pressure_2(start_valve_index, 26, 0, 0, &mut max_pressure_for_mask);

    let mut overall_max_pressure = 0;

    // itera over all possible masks for the ME
    for (mask_a, pressure_a) in max_pressure_for_mask.iter() {
        // elephant
        for (mask_b, pressure_b) in max_pressure_for_mask.iter() {
            // sets of opened valves cannot overlap
            if (mask_a & mask_b) == 0 {
                let total_pressure = pressure_a + pressure_b;
                overall_max_pressure = max(overall_max_pressure, total_pressure);
            }
        }
    }

    println!("{overall_max_pressure}")
}
