use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    first_unique_window(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    first_unique_window(input, 14)
}

fn first_unique_window(input: &str, win_size: usize) -> Option<usize> {
    let chars = input.chars().collect_vec();
    let mut iter = chars.windows(win_size);
    for i in win_size..input.len() {
        let next_set: HashSet<&char> = HashSet::from_iter(iter.next().unwrap());
        if next_set.len() == win_size {
            return Some(i);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
