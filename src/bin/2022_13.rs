use std::{cmp::Ordering, str::Chars};

use Packet::*;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Data(char),
    Inner(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data(c1), Data(c2)) => c1.cmp(c2),
            (Data(c), Inner(_)) => Inner(vec![Data(*c)]).cmp(other),
            (Inner(_), Data(c)) => self.cmp(&Inner(vec![Data(*c)])),
            (Inner(p1), Inner(p2)) => p1.cmp(p2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // normalize input, so we can use chars and rely on default ascii ordering
    let normalized_input = input.replace("10", "A");
    let pairs: Vec<&str> = normalized_input.split("\n\n").collect();
    let mut indexes: Vec<usize> = Vec::new();
    for (i, pair) in pairs.iter().enumerate() {
        let (left, right) = pair.split_once("\n").unwrap();
        let (left, right) = (
            build_packet(&mut left.chars()),
            build_packet(&mut right.chars()),
        );
        if left <= right {
            indexes.push(i + 1);
        }
    }
    Some(indexes.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let normalized_input = input.replace("10", "A");
    let mut packets: Vec<Packet> = normalized_input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }
            Some(build_packet(&mut l.chars()))
        })
        .collect();
    packets.sort();
    let div_2 = Inner(vec![Inner(vec![Data('2')])]);
    let div_6 = Inner(vec![Inner(vec![Data('6')])]);
    let index_2 = match packets.binary_search(&div_2) {
        Ok(_) => unreachable!("this element should not exist"),
        Err(i) => i,
    };
    let index_6 = match packets.binary_search(&div_6) {
        Ok(_) => unreachable!("this element should not exist"),
        Err(i) => i,
    };
    Some((index_2 + 1) * (index_6 + 2))
}

fn build_packet(chars: &mut Chars) -> Packet {
    let mut packets = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                packets.push(build_packet(chars));
            }
            ']' => break,
            ',' => {}
            _ => {
                packets.push(Data(c));
            }
        }
    }
    Inner(packets)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
