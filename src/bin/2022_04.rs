use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first_range = pairs[0];
        let second_range = pairs[1];
        let first_sections: Vec<&str> = first_range.split("-").collect();
        let second_sections: Vec<&str> = second_range.split("-").collect();
        let f_start = first_sections[0].parse::<u32>().unwrap();
        let f_end = first_sections[1].parse::<u32>().unwrap();
        let s_start = second_sections[0].parse::<u32>().unwrap();
        let s_end = second_sections[1].parse::<u32>().unwrap();
        let f_set: HashSet<u32> = HashSet::from_iter(f_start..=f_end);
        let s_set: HashSet<u32> = HashSet::from_iter(s_start..=s_end);
        if f_set.difference(&s_set).sum::<u32>() == 0 {
            sum += 1;
        } else if s_set.difference(&f_set).sum::<u32>() == 0 {
            sum += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let pairs: Vec<&str> = line.split(",").collect();
        let first_range = pairs[0];
        let second_range = pairs[1];
        let first_sections: Vec<&str> = first_range.split("-").collect();
        let second_sections: Vec<&str> = second_range.split("-").collect();
        let f_start = first_sections[0].parse::<u32>().unwrap();
        let f_end = first_sections[1].parse::<u32>().unwrap();
        let s_start = second_sections[0].parse::<u32>().unwrap();
        let s_end = second_sections[1].parse::<u32>().unwrap();
        let f_set: HashSet<u32> = HashSet::from_iter(f_start..=f_end);
        let s_set: HashSet<u32> = HashSet::from_iter(s_start..=s_end);
        if f_set.intersection(&s_set).sum::<u32>() > 0 {
            sum += 1;
        }
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
