use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

pub fn part_one(input: &str) -> Option<u64> {
    populate_bit_valves(input);
    Some(score(pack_state(0, 30, 0, 0)))
}

pub fn part_two(input: &str) -> Option<u64> {
    populate_bit_valves(input);
    Some(score(pack_state(0, 26, 0, 1)))
}

fn populate_bit_valves(input: &str) {
    let string_valves: BTreeMap<String, Valve<String>> = input
        .lines()
        .map(|l| {
            let v = Valve::from_str(l).unwrap();
            (v.name.clone(), v)
        })
        .collect();
    let valve_indexes: BTreeMap<String, u64> = string_valves
        .iter()
        .enumerate()
        .map(|(i, (s, _))| (s.clone(), i as u64))
        .collect();
    for (k, v) in string_valves.iter() {
        let name = valve_indexes[k];
        let conns: Vec<u64> = v.conns.iter().map(|c| valve_indexes[c]).collect();
        BIT_VALVES.lock().unwrap().insert(
            name,
            Valve {
                name,
                rate: v.rate,
                conns,
            },
        );
    }
}

static BIT_VALVES: Lazy<Mutex<BTreeMap<u64, Valve<u64>>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));
static STATES: Lazy<Mutex<HashMap<u64, u64>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static BIT_MASKS: Lazy<HashMap<u64, u64>> = Lazy::new(|| (0..64).map(|i| (i, 1 << i)).collect());
// static BIT_CLEARS: Lazy<HashMap<u64, u64>> =
//     Lazy::new(|| (0..64).map(|i| (i, (1 << i) ^ 0)).collect());

const VALVES_MASK: u64 = 2_u64.pow(51) - 1;
const TIME_SHIFT: u64 = 51;
const TIME_MASK: u64 = 2_u64.pow(5) - 1;
const POSITION_SHIFT: u64 = 56;
const POSITION_MASK: u64 = 2_u64.pow(6) - 1;
const ELEPHANT_SHIFT: u64 = 62;
const ELEPHANT_MASK: u64 = 2_u64.pow(1) - 1;

fn set_bit(state: &u64, index: &u64) -> u64 {
    state | BIT_MASKS[index]
}

fn get_bit(state: &u64, index: &u64) -> u64 {
    state & BIT_MASKS[index]
}

fn has_bit(state: &u64, index: &u64) -> bool {
    get_bit(state, index) > 0
}

// fn clear_bit(state: &u64, index: &u64) -> u64 {
//     state & BIT_CLEARS[index]
// }

fn unpack_state(state: &u64) -> (u64, u64, u64, u64) {
    (
        (state >> POSITION_SHIFT) & POSITION_MASK,
        (state >> TIME_SHIFT) & TIME_MASK,
        state & VALVES_MASK,
        (state >> ELEPHANT_SHIFT) & ELEPHANT_MASK,
    )
}

fn pack_state(position: u64, time: u64, valves: u64, elephant: u64) -> u64 {
    let position_shifted = position << POSITION_SHIFT;
    let time_shifted = time << TIME_SHIFT;
    let elephant_shifted = elephant << ELEPHANT_SHIFT;
    position_shifted | time_shifted | valves | elephant_shifted
}

#[derive(Debug)]
struct Valve<T> {
    name: T,
    rate: u64,
    conns: Vec<T>,
}

impl FromStr for Valve<String> {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let name: String = words[1].into();
        let rate = words[4]
            .split_once("=")
            .unwrap()
            .1
            .strip_suffix(";")
            .unwrap()
            .parse()
            .unwrap();
        let conns: Vec<String> = words[9..]
            .iter()
            .map(|w| w.trim_end_matches(",").into())
            .collect();
        Ok(Valve { name, rate, conns })
    }
}

fn score(state: u64) -> u64 {
    let (position, time, valves, elephant) = unpack_state(&state);
    if time == 0 {
        return match elephant > 0 {
            false => 0,
            true => score(pack_state(0, 26, valves, 0)),
        };
    }
    if let Some(&cached_score) = STATES.lock().unwrap().get(&state) {
        return cached_score;
    }
    let conns: Vec<u64> = BIT_VALVES
        .lock()
        .unwrap()
        .get(&position)
        .unwrap()
        .conns
        .clone();
    let mut scores: Vec<u64> = conns
        .iter()
        .map(|c| score(pack_state(*c, time - 1, valves, elephant)))
        .collect();
    let rate = BIT_VALVES.lock().unwrap().get(&position).unwrap().rate;
    if rate > 0 && !has_bit(&valves, &position) {
        let new_valves = set_bit(&valves, &position);
        scores
            .push(rate * (time - 1) + score(pack_state(position, time - 1, new_valves, elephant)));
    }
    let max_score = scores.iter().max().copied().unwrap();
    STATES.lock().unwrap().insert(state, max_score);
    max_score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
