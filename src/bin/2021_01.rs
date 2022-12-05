use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut prev: Option<u32> = None;
    let mut sum: u32 = 0;
    for line in lines {
        let cur_depth = line.parse::<u32>().unwrap();
        if let Some(prev_depth) = prev {
            if cur_depth > prev_depth {
                sum += 1;
            }
        }
        prev = Some(cur_depth);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut prev: Option<u32> = None;
    let mut sum: u32 = 0;
    let mut window: VecDeque<u32> = VecDeque::new();
    while let Some(line) = lines.next() {
        let cur_depth = line.parse::<u32>().unwrap();
        window.push_back(cur_depth);
        if window.len() < 3 {
            continue;
        }
        let sliding_depth: u32 = window.iter().sum();
        if let Some(prev_depth) = prev {
            if sliding_depth > prev_depth {
                sum += 1;
            }
        }
        prev = Some(sliding_depth);
        window.pop_front();
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2021, 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2021, 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2021, 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
