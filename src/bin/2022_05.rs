use std::collections::HashMap;

enum Phase {
    STACKS,
    MOVES,
}

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines();
    let mut phase = Phase::STACKS;
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for line in lines {
        if line.starts_with("m") {
            phase = Phase::MOVES;
        }
        match phase {
            Phase::STACKS => {
                fill_stacks(line, &mut stacks);
            }
            Phase::MOVES => {
                move_crates(line, &mut stacks, false);
            }
        }
    }
    Some(top_crates(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input.lines();
    let mut phase = Phase::STACKS;
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    for line in lines {
        if line.starts_with("m") {
            phase = Phase::MOVES;
        }
        match phase {
            Phase::STACKS => {
                fill_stacks(line, &mut stacks);
            }
            Phase::MOVES => {
                move_crates(line, &mut stacks, true);
            }
        }
    }
    Some(top_crates(&stacks))
}

fn top_crates(stacks: &HashMap<usize, Vec<char>>) -> String {
    let mut next_index = 1;
    let mut tops: String = "".to_string();
    while let Some(stack) = stacks.get(&next_index) {
        tops += &stack.last().unwrap().to_string();
        next_index += 1;
    }
    tops
}

fn fill_stacks(line: &str, stacks: &mut HashMap<usize, Vec<char>>) {
    let chars = line.chars();
    let mut stack_num = 0;
    for (i, c) in chars.enumerate() {
        if i % 4 == 1 {
            stack_num += 1;
            if !c.is_alphabetic() {
                continue;
            }
            if stacks.contains_key(&stack_num) {
                let stack = stacks.get_mut(&stack_num).unwrap();
                stack.insert(0, c);
            } else {
                stacks.insert(stack_num, vec![c]);
            }
        }
    }
}

fn move_crates(line: &str, stacks: &mut HashMap<usize, Vec<char>>, move_multi: bool) {
    let parts: Vec<&str> = line.split(" ").collect();
    let amount = parts[1].parse::<usize>().unwrap();
    let stack_a = stacks.get_mut(&parts[3].parse::<usize>().unwrap()).unwrap();
    let mut temp: Vec<char> = Vec::new();
    for _ in 0..amount {
        let c = stack_a.pop().unwrap();
        temp.push(c);
    }
    if move_multi {
        temp.reverse();
    }
    let stack_b = stacks.get_mut(&parts[5].parse::<usize>().unwrap()).unwrap();
    for c in temp {
        stack_b.push(c);
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
