use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
};
use Tile::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    EMPTY,
    ROCK,
    SAND,
}

#[derive(Debug)]
struct Grid {
    map: Vec<Tile>,
    width: i32,
    height: i32,
    sand_source: Point,
}

impl Grid {
    fn new(input: &str, infinite_floor: bool) -> Self {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_max = i32::MIN;
        let mut rock_points: HashSet<Point> = HashSet::new();
        for path in input.lines() {
            let points = path.split(" -> ");
            let mut prev: Option<Point> = None;
            for point_raw in points {
                let (x, y) = point_raw.split_once(",").unwrap();
                let current_point =
                    Point::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap());
                x_min = x_min.min(current_point.x);
                x_max = x_max.max(current_point.x);
                y_max = y_max.max(current_point.y);
                if let Some(prev_point) = prev {
                    let dir = current_point - prev_point;
                    if dir.x > 0 {
                        for i in 0..=dir.x {
                            rock_points.insert(prev_point + Point::new(i, 0));
                        }
                    } else if dir.x < 0 {
                        for i in 0..=dir.x.abs() {
                            rock_points.insert(prev_point + Point::new(-i, 0));
                        }
                    } else if dir.y > 0 {
                        for i in 0..=dir.y {
                            rock_points.insert(prev_point + Point::new(0, i));
                        }
                    } else if dir.y < 0 {
                        for i in 0..=dir.y.abs() {
                            rock_points.insert(prev_point + Point::new(0, -i));
                        }
                    } else {
                        unreachable!("invalid direction {:?}", dir);
                    }
                }
                prev = Some(current_point);
            }
        }
        let mut width = x_max - x_min + 1;
        let mut height = y_max + 1;
        if infinite_floor {
            width = x_max * 2;
            height += 2;
        }
        let mut map: Vec<Tile> = vec![EMPTY; (width * height) as usize];
        let mut sand_source = Point::new(500 - x_min, 0);
        if infinite_floor {
            sand_source = Point::new(500, 0);
        }
        for rock_point in rock_points {
            let mut index = (rock_point.y * width + rock_point.x - x_min) as usize;
            if infinite_floor {
                index = (rock_point.y * width + rock_point.x) as usize;
            }
            map[index] = ROCK;
        }
        if infinite_floor {
            map[(sand_source.y * width + sand_source.x) as usize] = SAND;
            for x in 0..width {
                let index = ((height - 1) * width + x) as usize;
                map[index] = ROCK;
            }
        }
        Grid {
            map,
            width,
            height,
            sand_source,
        }
    }

    fn get(&self, p: &Point) -> Option<Tile> {
        self.map.get((p.y * self.width + p.x) as usize).copied()
    }

    fn simulate(&mut self) {
        let mut next_point = self.sand_source + Point::new(0, 1);

        let mut safety = 0;
        while let Some(tile) = self.get(&next_point) {
            match tile {
                EMPTY => {
                    next_point = next_point + Point::new(0, 1);
                }
                _ => {
                    let left_point = next_point + Point::new(-1, 0);
                    let right_point = next_point + Point::new(1, 0);
                    let left = self.get(&left_point);
                    let right = self.get(&right_point);
                    match (left, right) {
                        // hit left edge of map, sand will fall endlessly
                        (None, Some(_)) => {
                            break;
                        }
                        // hit right edge of map, sand will fall endlessly
                        (Some(_), None) => {
                            break;
                        }
                        // left point is empty
                        (Some(EMPTY), Some(_)) => {
                            next_point = left_point;
                        }
                        // left is occupied and right point is empty
                        (Some(_), Some(EMPTY)) => {
                            next_point = right_point;
                        }
                        // left and right are occupied
                        (Some(_), Some(_)) => {
                            self.map[((next_point.y - 1) * self.width + next_point.x) as usize] =
                                SAND;
                            next_point = self.sand_source + Point::new(0, 1);
                        }
                        (None, None) => {
                            unreachable!("both left and right points should not be None");
                        }
                    }
                }
            }
            safety += 1;
            if safety > 10_000_000 {
                break;
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(&Point::new(x, y)).unwrap() {
                    EMPTY => {
                        write!(f, ".")?;
                    }
                    ROCK => {
                        write!(f, "#")?;
                    }
                    SAND => {
                        write!(f, "+")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::new(input, false);
    grid.simulate();
    println!("{}", grid);
    Some(grid.map.iter().filter(|&t| t == &SAND).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::new(input, true);
    grid.simulate();
    // println!("{}", grid);
    Some(grid.map.iter().filter(|&t| t == &SAND).count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
