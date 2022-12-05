use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect_vec()
            .windows(2)
            .filter(|win| win[0] < win[1])
            .collect_vec()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect_vec()
            .windows(4)
            .filter(|win| win[0] < win[3])
            .collect_vec()
            .len(),
    )
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
