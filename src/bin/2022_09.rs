use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
enum Direction {
    R(u32),
    L(u32),
    U(u32),
    D(u32),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(" ").unwrap();
        match dir {
            "R" => Ok(Direction::R(steps.parse::<u32>().unwrap())),
            "L" => Ok(Direction::L(steps.parse::<u32>().unwrap())),
            "U" => Ok(Direction::U(steps.parse::<u32>().unwrap())),
            "D" => Ok(Direction::D(steps.parse::<u32>().unwrap())),
            _ => Err(anyhow!("invalid dir")),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(unique_tail_positions(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(unique_tail_positions(input, 10))
}

fn unique_tail_positions(input: &str, rope_length: usize) -> usize {
    let mut unique_positions: HashSet<Position> = HashSet::new();
    let motions: Vec<Direction> = input
        .lines()
        .map(|l| Direction::from_str(l).unwrap())
        .collect();
    let mut rope: Vec<Position> = Vec::new();
    for _ in 0..rope_length {
        rope.push(Position::new(0, 0));
    }
    unique_positions.insert(Position::new(0, 0));
    for motion in motions {
        match motion {
            Direction::R(steps) => {
                move_rope(&mut unique_positions, &mut rope, Position::new(1, 0), steps)
            }
            Direction::L(steps) => move_rope(
                &mut unique_positions,
                &mut rope,
                Position::new(-1, 0),
                steps,
            ),
            Direction::U(steps) => {
                move_rope(&mut unique_positions, &mut rope, Position::new(0, 1), steps)
            }
            Direction::D(steps) => move_rope(
                &mut unique_positions,
                &mut rope,
                Position::new(0, -1),
                steps,
            ),
        }
    }
    unique_positions.len()
}

static FOLLOW_DIRS: Lazy<HashMap<Position, Position>> = Lazy::new(|| {
    let mut follow_dirs: HashMap<Position, Position> = HashMap::new();
    follow_dirs.insert(Position::new(2, 0), Position::new(1, 0));
    follow_dirs.insert(Position::new(-2, 0), Position::new(-1, 0));
    follow_dirs.insert(Position::new(0, 2), Position::new(0, 1));
    follow_dirs.insert(Position::new(0, -2), Position::new(0, -1));
    follow_dirs.insert(Position::new(1, 2), Position::new(1, 1));
    follow_dirs.insert(Position::new(2, 2), Position::new(1, 1));
    follow_dirs.insert(Position::new(2, 1), Position::new(1, 1));
    follow_dirs.insert(Position::new(1, -2), Position::new(1, -1));
    follow_dirs.insert(Position::new(2, -2), Position::new(1, -1));
    follow_dirs.insert(Position::new(2, -1), Position::new(1, -1));
    follow_dirs.insert(Position::new(-1, 2), Position::new(-1, 1));
    follow_dirs.insert(Position::new(-2, 2), Position::new(-1, 1));
    follow_dirs.insert(Position::new(-2, 1), Position::new(-1, 1));
    follow_dirs.insert(Position::new(-1, -2), Position::new(-1, -1));
    follow_dirs.insert(Position::new(-2, -2), Position::new(-1, -1));
    follow_dirs.insert(Position::new(-2, -1), Position::new(-1, -1));
    follow_dirs
});

fn move_rope(
    unique_positions: &mut HashSet<Position>,
    rope: &mut Vec<Position>,
    dir: Position,
    steps: u32,
) {
    for _ in 0..steps {
        let head = rope.last_mut().unwrap();
        *head = head.add(dir);
        for i in (1..rope.len()).rev() {
            let delta = rope[i].sub(rope[i - 1]);
            if let Some(follow_dir) = FOLLOW_DIRS.get(&delta) {
                let tail_knot = rope.get_mut(i - 1).unwrap();
                *tail_knot = tail_knot.add(*follow_dir);
            } else {
                break;
            }
        }
        unique_positions.insert(*rope.first().unwrap());
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGER_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 9);
        assert_eq!(part_two(&input), Some(1));
        assert_eq!(part_two(LARGER_EXAMPLE), Some(36));
    }
}
