pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let mut chars = line.split(' ');
        let opp_move = chars.next().unwrap();
        let my_move = chars.next().unwrap();
        score += score_round(opp_move, my_move)
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut score = 0;
    for line in lines {
        let mut chars = line.split(' ');
        let opp_move = chars.next().unwrap();
        let my_move = chars.next().unwrap();
        score += score_round_2(opp_move, my_move)
    }
    Some(score)
}

fn score_round(a: &str, b: &str) -> u32 {
    let mut score = 0;

    match b {
        "X" => {
            score += 1;
            match a {
                "A" => score += 3,
                "C" => score += 6,
                _ => {}
            }
        }
        "Y" => {
            score += 2;
            match a {
                "B" => score += 3,
                "A" => score += 6,
                _ => {}
            }
        }
        "Z" => {
            score += 3;
            match a {
                "C" => score += 3,
                "B" => score += 6,
                _ => {}
            }
        }
        _ => {}
    }

    score
}

fn score_round_2(a: &str, b: &str) -> u32 {
    let mut score = 0;

    match b {
        "X" => match a {
            "A" => score += 3,
            "B" => score += 1,
            "C" => score += 2,
            _ => {}
        },
        "Y" => {
            score += 3;
            match a {
                "A" => score += 1,
                "B" => score += 2,
                "C" => score += 3,
                _ => {}
            }
        }
        "Z" => {
            score += 6;
            match a {
                "A" => score += 2,
                "B" => score += 3,
                "C" => score += 1,
                _ => {}
            }
        }
        _ => {}
    }

    score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
