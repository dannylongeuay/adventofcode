use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    find_cost(&grid, vec![grid.start], grid.end)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::new(input);
    let starts: Vec<(isize, isize)> = (0..grid.width)
        .cartesian_product(0..grid.height)
        .filter_map(|pos| match grid.get(pos) {
            0 => Some(pos),
            _ => None,
        })
        .collect();
    find_cost(&grid, starts, grid.end)
}

#[derive(Debug)]
struct Grid {
    map: Vec<isize>,
    start: (isize, isize),
    end: (isize, isize),
    width: isize,
    height: isize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut map: Vec<isize> = Vec::new();
        let mut start: (isize, isize) = (-1, -1);
        let mut end: (isize, isize) = (-1, -1);
        let mut width = 0;
        let mut height = 0;
        for (y, l) in input.lines().enumerate() {
            width = l.chars().count();
            for (x, c) in l.chars().enumerate() {
                match c {
                    'S' => {
                        map.push(0);
                        start = (x as isize, y as isize);
                    }
                    'E' => {
                        map.push(25);
                        end = (x as isize, y as isize);
                    }
                    _ => map.push((c.to_digit(36).unwrap() - 10) as isize),
                }
            }
            height += 1;
        }
        Grid {
            map,
            start,
            end,
            width: width as isize,
            height,
        }
    }

    fn get(&self, pos: (isize, isize)) -> isize {
        assert!(self.in_bounds(pos));
        self.map[(pos.1 * self.width + pos.0) as usize]
    }

    fn in_bounds(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.width && pos.1 < self.height
    }

    fn neighbors(&self, pos: (isize, isize)) -> Vec<(isize, isize)> {
        let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut result: Vec<(isize, isize)> = Vec::new();
        let current_elevation = self.get(pos);
        for dir in dirs {
            let neighbor_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !self.in_bounds(neighbor_pos) {
                continue;
            }
            let neighbor_elevation = self.get(neighbor_pos);
            if neighbor_elevation - current_elevation > 1 {
                continue;
            }
            result.push(neighbor_pos);
        }
        result
    }
}

fn find_cost(grid: &Grid, starts: Vec<(isize, isize)>, end: (isize, isize)) -> Option<usize> {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    let mut costs: HashMap<(isize, isize), usize> = HashMap::new();
    for start in starts {
        queue.push_back(start);
        costs.insert(start, 0);
    }
    while let Some(current_pos) = queue.pop_front() {
        if current_pos == end {
            break;
        }
        for neighbor_pos in grid.neighbors(current_pos) {
            if costs.contains_key(&neighbor_pos) {
                continue;
            }
            let neighbor_cost = costs.get(&current_pos).unwrap() + 1;
            costs.insert(neighbor_pos, neighbor_cost);
            queue.push_back(neighbor_pos);
        }
    }
    costs.get(&end).copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
