use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut best_sum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            sum = 0;
            continue;
        }
        let n = line.to_string().parse::<u32>().unwrap();
        sum += n;
        if sum > best_sum {
            best_sum = sum;
        }
    }
    Some(best_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut sum: u32 = 0;
    for line in lines {
        if line.is_empty() {
            heap.push(sum);
            sum = 0;
            continue;
        }
        let n = line.to_string().parse::<u32>().unwrap();
        sum += n;
    }
    heap.push(sum);
    let mut total_sum = heap.pop().unwrap();
    total_sum += heap.pop().unwrap();
    total_sum += heap.pop().unwrap();
    Some(total_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
