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

#[derive(Debug, Clone, Copy)]
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
    fn new(input: &str) -> Self {
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
        let width = x_max - x_min + 1;
        let height = y_max + 1;
        let mut map: Vec<Tile> = vec![EMPTY; (width * height) as usize];
        let sand_source = Point::new(500 - x_min, 0);
        for rock_point in rock_points {
            map[(rock_point.y * width + rock_point.x - x_min) as usize] = ROCK;
        }
        map[(sand_source.y * width + sand_source.x) as usize] = SAND;
        Grid {
            map,
            width,
            height,
            sand_source,
        }
    }

    fn get(&self, p: Point) -> Tile {
        self.map[(p.y * self.width + p.x) as usize]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(Point::new(x, y)) {
                    EMPTY => {
                        write!(f, " ")?;
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

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    println!("{}", grid);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
