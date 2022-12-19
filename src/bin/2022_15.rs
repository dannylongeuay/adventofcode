use anyhow::Error;
use std::{collections::HashSet, str::FromStr};

pub fn part_one(input: &str) -> Option<u32> {
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|l| Sensor::from_str(l).unwrap())
        .collect();
    // example row
    let mut row: i32 = 10;
    if sensors.len() > 14 {
        // input row
        row = 2_000_000;
    }
    let beacons_on_row: HashSet<(i32, i32)> = sensors
        .iter()
        .filter(|s| s.beacon.1 == row)
        .map(|s| s.beacon)
        .collect();
    Some(
        (merged_ranges(&sensors, row)
            .iter()
            .fold(0, |acc, r| acc + r.end - r.start + 1)
            - beacons_on_row.len() as i32) as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Sensor {
    loc: (i32, i32),
    beacon: (i32, i32),
    distance: u32,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(":").unwrap();
        let sensor = parse_coordinate(first);
        let beacon = parse_coordinate(second);
        let distance = manhattan_distance(sensor, beacon);
        Ok(Sensor {
            loc: sensor,
            beacon,
            distance,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }

    fn contains(&self, value: i32) -> bool {
        value >= self.start && value <= self.end
    }
}

fn merged_ranges(sensors: &Vec<Sensor>, row: i32) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();
    let mut merged_ranges: Vec<Range> = Vec::with_capacity(sensors.len());
    for sensor in sensors {
        if let Some(r) = sensor_range(sensor, row) {
            ranges.push(r);
        }
    }
    ranges.sort_by_key(|r| r.start);
    if ranges.is_empty() {
        return merged_ranges;
    }
    let mut index = 0;
    let mut r1_opt = ranges.get(index).copied();
    loop {
        let r2_opt = ranges.get(index).copied();
        index += 1;
        match (r1_opt, r2_opt) {
            (Some(r1), None) => {
                merged_ranges.push(r1);
                break;
            }
            (Some(r1), Some(r2)) => {
                if r1.contains(r2.start) || r1.end + 1 == r2.start {
                    r1_opt = Some(Range::new(r1.start, r1.end.max(r2.end)));
                } else {
                    r1_opt = Some(r2);
                    merged_ranges.push(r1);
                }
            }
            (None, _) => break,
        }
    }
    merged_ranges
}

fn sensor_range(sensor: &Sensor, row: i32) -> Option<Range> {
    let h = sensor.distance as i32 - (row - sensor.loc.1).abs();
    if h <= 0 {
        return None;
    }
    Some(Range::new(sensor.loc.0 - h, sensor.loc.0 + h))
}

fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u32
}

fn parse_coordinate(s: &str) -> (i32, i32) {
    let (first, second) = s.split_once(", ").unwrap();
    let first_terms = first.split_whitespace();
    let second_terms = second.split_whitespace();
    let x_coord = parse_axis(first_terms.last().unwrap());
    let y_coord = parse_axis(second_terms.last().unwrap());
    (x_coord, y_coord)
}

fn parse_axis(s: &str) -> i32 {
    s.split_once("=").unwrap().1.parse::<i32>().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 15);
        assert_eq!(part_two(&input), None);
    }
}
