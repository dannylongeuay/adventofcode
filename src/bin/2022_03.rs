pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let compartments = line.split_at(line.len() / 2);
        let first = compartments.0;
        let second = compartments.1;
        for c in first.chars() {
            if second.contains(c) {
                let mut priority = c.to_digit(36).unwrap() - 9;
                if c.is_uppercase() {
                    priority += 26;
                }
                sum += priority;
                break;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut group: Vec<&str> = Vec::new();
    for line in lines {
        group.push(line);
        if group.len() < 3 {
            continue;
        }
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                let mut priority = c.to_digit(36).unwrap() - 9;
                if c.is_uppercase() {
                    priority += 26;
                }
                sum += priority;
                break;
            }
        }
        group.clear();
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
