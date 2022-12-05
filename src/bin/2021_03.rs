use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let common_bits =
        input
            .lines()
            .map(|l| l.chars())
            .fold(vec![], |mut accum: Vec<Vec<char>>, item| {
                for (i, c) in item.enumerate() {
                    if accum.len() == i {
                        accum.push(vec![]);
                    }
                    accum[i].push(c);
                }
                accum
            });
    let most_common_bits: String = common_bits
        .iter()
        .map(|b| {
            let counts = b.iter().counts();
            if counts[&'0'] > counts[&'1'] {
                return "0";
            }
            return "1";
        })
        .collect();
    let least_common_bits: String = common_bits
        .iter()
        .map(|b| {
            let counts = b.iter().counts();
            if counts[&'0'] < counts[&'1'] {
                return "0";
            }
            return "1";
        })
        .collect();
    let gamma_rate = u32::from_str_radix(&most_common_bits, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&least_common_bits, 2).unwrap();
    Some(gamma_rate * epsilon_rate)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ox_lines: Vec<&str> = input.lines().collect();
    let mut co_lines = ox_lines.clone();
    let mut ox_idx = 0;
    while ox_lines.len() > 1 {
        let common_bits =
            ox_lines
                .iter()
                .map(|l| l.chars())
                .fold(vec![], |mut accum: Vec<Vec<char>>, item| {
                    for (i, c) in item.enumerate() {
                        if accum.len() == i {
                            accum.push(vec![]);
                        }
                        accum[i].push(c);
                    }
                    accum
                });
        let counts = common_bits[ox_idx].iter().counts();
        let mut common_bit = '1';
        if counts[&'0'] > counts[&'1'] {
            common_bit = '0';
        }
        ox_lines = ox_lines
            .iter()
            .cloned()
            .filter(|l| l.chars().collect::<Vec<char>>()[ox_idx] == common_bit)
            .collect();
        ox_idx += 1;
    }
    let mut co_idx = 0;
    while co_lines.len() > 1 {
        let common_bits =
            co_lines
                .iter()
                .map(|l| l.chars())
                .fold(vec![], |mut accum: Vec<Vec<char>>, item| {
                    for (i, c) in item.enumerate() {
                        if accum.len() == i {
                            accum.push(vec![]);
                        }
                        accum[i].push(c);
                    }
                    accum
                });
        let counts = common_bits[co_idx].iter().counts();
        let mut common_bit = '0';
        if counts[&'0'] > counts[&'1'] {
            common_bit = '1';
        }
        co_lines = co_lines
            .iter()
            .cloned()
            .filter(|l| l.chars().collect::<Vec<char>>()[co_idx] == common_bit)
            .collect();
        co_idx += 1;
    }
    let ox_rate = u32::from_str_radix(&ox_lines[0], 2).unwrap();
    let co_rate = u32::from_str_radix(&co_lines[0], 2).unwrap();
    Some(ox_rate * co_rate)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2021, 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2021, 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2021, 3);
        assert_eq!(part_two(&input), Some(230));
    }
}
