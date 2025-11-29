use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use regex::Regex;

/// Advent of Code 2022 - Day 15
/// https://adventofcode.com/2022/day/15
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

type Coords = (i32, i32);

fn visualize(
    exam_row_y: i32,
    sensors: &Vec<Coords>,
    beacons: &Vec<Coords>,
    off_positions: &HashSet<i32>,
) {
    let mut all = sensors.clone();
    all.extend(beacons);
    let min_bounds = all
        .iter()
        .fold((i32::MAX, i32::MAX), |(min_x, min_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y))
        });
    let max_bounds = all.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });

    let width = (max_bounds.0 - min_bounds.0) + 1;
    let height = (max_bounds.1 - min_bounds.1) + 1;

    for y in min_bounds.1..=height {
        print!("{:3} ", y);
        for x in (min_bounds.0)..=width {
            let c = (min_bounds.0 + x, min_bounds.1 + y);
            let at_sensor = sensors.iter().any(|(rx, ry)| *rx == c.0 && *ry == c.1);
            let at_beacon = beacons.iter().any(|(rx, ry)| *rx == c.0 && *ry == c.1);
            let at_off = off_positions.iter().any(|rx| *rx == c.0) && c.1 == exam_row_y;

            if at_sensor {
                print!("S");
            } else if at_beacon {
                print!("B");
            } else if at_off {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn manhattan_distance((x1, y1): Coords, (x2, y2): Coords) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut sensors: Vec<Coords> = Vec::new();
    let mut beacons: Vec<Coords> = Vec::new();
    let examine_y = 2000000;
    let mut off_positions = HashSet::new();
    for line in lines {
        let (_, [sx_str, sy_str, bx_str, by_str]) = re.captures(line).unwrap().extract();
        let sx = sx_str.parse::<i32>().unwrap();
        let sy = sy_str.parse::<i32>().unwrap();
        let bx = bx_str.parse::<i32>().unwrap();
        let by = by_str.parse::<i32>().unwrap();
        let sensor = (sx, sy);
        let beacon = (bx, by);
        sensors.push(sensor);
        beacons.push(beacon);

        let dist = manhattan_distance(sensor, beacon);

        // check if sensor's range intersects with examined row
        let range = (sensor.1 - dist)..(sensor.1 + dist);
        if range.contains(&examine_y) {
            // the start of the examined row is the sensor's x minus the distance minus the height difference between sensor and examined row
            let start_x_in_row = sensor.0 - (dist - examine_y.abs_diff(sensor.1) as i32);
            let end_x_in_row = sensor.0 + (dist - examine_y.abs_diff(sensor.1) as i32);
            off_positions.extend(start_x_in_row..=end_x_in_row);
        }
    }
    // of the given row, a sensor takes "manhattan_distance + height_difference + 1" space into the row

    // visualize(examine_y, &sensors, &beacons, &off_positions);
    println!("{}", off_positions.len() - 1);
}

#[allow(unused)]
fn part2(input: &str) {
    fn coords_range(start: Coords, end: Coords) -> Vec<Coords> {
        let mut coords = Vec::new();
        for x in start.0..end.0 {
            for y in start.1..end.1 {
                coords.push((x, y));
            }
        }
        coords
    }
    #[derive(Debug, Clone, Copy)]
    struct ScanData {
        sensor: Coords,
        beacon: Coords,
        distance: i32,
    }

    let lines = input.lines();
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let min_bounds = (0, 0);
    let max_bounds = (4000000, 4000000);
    fn is_hidden_beacon(
        coord: Coords,
        min_bounds: (i32, i32),
        max_bounds: (i32, i32),
        scans: &Vec<ScanData>,
    ) -> bool {
        let is_out_of_bounds = coord.0 < min_bounds.0
            || coord.0 > max_bounds.0
            || coord.1 < min_bounds.1
            || coord.1 > max_bounds.1;

        let mut overlaps = false;
        for scan in scans {
            if manhattan_distance(coord, scan.sensor) <= scan.distance {
                overlaps = true;
                break;
            }
        }
        !is_out_of_bounds && !overlaps
    }

    let mut scans = Vec::<ScanData>::new();
    for line in lines {
        let (_, [sx_str, sy_str, bx_str, by_str]) = re.captures(line).unwrap().extract();
        let sx = sx_str.parse::<i32>().unwrap();
        let sy = sy_str.parse::<i32>().unwrap();
        let bx = bx_str.parse::<i32>().unwrap();
        let by = by_str.parse::<i32>().unwrap();
        let sensor = (sx, sy);
        let beacon = (bx, by);
        let dist = manhattan_distance(sensor, beacon);
        scans.push(ScanData {
            sensor,
            beacon,
            distance: dist,
        });
    }

    for scan in scans.clone() {
        let sensor = scan.sensor;
        let beacon = scan.beacon;
        let dist = scan.distance + 1;

        let top_vertex = (sensor.0, sensor.1 - dist);
        let right_vertex = (sensor.0 + dist, sensor.1);
        let bottom_vertex = (sensor.0, sensor.1 + dist);
        let left_vertex = (sensor.0 - dist, sensor.1);

        // top-right segment
        for x in sensor.0..sensor.0 + dist {
            let y = sensor.1 - dist + (x - sensor.0);
            let coord = (x, y);
            if is_hidden_beacon(coord, min_bounds, max_bounds, &scans) {
                println!("Hidden beacon found at {coord:?}");
                let tuning_freq = coord.0 as u64 * max_bounds.0 as u64 + coord.1 as u64;
                println!("{}", tuning_freq);
                return;
            }
        }
        // bottom-right segment
        for x in (sensor.0..sensor.1 + dist).rev() {
            let y = sensor.1 + dist - (x - sensor.0);
            let coord = (x, y);
            if is_hidden_beacon(coord, min_bounds, max_bounds, &scans) {
                println!("Hidden beacon found at {coord:?}");
                let tuning_freq = coord.0 as u64 * max_bounds.0 as u64 + coord.1 as u64;
                println!("{}", tuning_freq);
                return;
            }
        }
        // top-left segment
        for x in (sensor.0 - dist)..sensor.0 {
            let y = sensor.1 - dist + (x - sensor.0);
            let coord = (x, y);
            if is_hidden_beacon(coord, min_bounds, max_bounds, &scans) {
                println!("Hidden beacon found at {coord:?}");
                let tuning_freq = coord.0 as u64 * max_bounds.0 as u64 + coord.1 as u64;
                println!("{}", tuning_freq);
                return;
            }
        }
        // bottom-left segment
        for x in (sensor.0 - dist)..sensor.0 {
            let y = sensor.1 + dist - (x - sensor.0);
            let coord = (x, y);
            if is_hidden_beacon(coord, min_bounds, max_bounds, &scans) {
                println!("Hidden beacon found at {coord:?}");
                let tuning_freq = coord.0 as u64 * max_bounds.0 as u64 + coord.1 as u64;
                println!("{}", tuning_freq);
                return;
            }
        }
        // 10852583132904
    }
}
