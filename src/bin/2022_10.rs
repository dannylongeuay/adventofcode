use std::collections::HashSet;

use once_cell::sync::Lazy;

static SIGNAL_CHECKS: Lazy<HashSet<i32>> = Lazy::new(|| {
    let checks: HashSet<i32> = HashSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
    checks
});

const WIDTH: i32 = 40;
const HEIGHT: i32 = 6;
const SPRITE_WIDTH: i32 = 3;

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|l| {
                if let Some((_cmd, val)) = l.split_once(" ") {
                    return Some(val.parse::<i32>().unwrap());
                }
                None
            })
            .fold((0, 1, 0), |acc: (i32, i32, i32), item| {
                let mut signal_strength = acc.0;
                let mut register = acc.1;
                let mut cycle = acc.2;
                if let Some(val) = item {
                    for i in 0..2 {
                        cycle += 1;
                        if SIGNAL_CHECKS.contains(&cycle) {
                            signal_strength += register * cycle;
                        }
                        if i == 1 {
                            register += val;
                        }
                    }
                } else {
                    cycle += 1;
                    if SIGNAL_CHECKS.contains(&cycle) {
                        signal_strength += register * cycle;
                    }
                }
                (signal_strength, register, cycle)
            })
            .0,
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut message = vec!["."; WIDTH as usize * HEIGHT as usize];
    input
        .lines()
        .map(|l| {
            if let Some((_cmd, val)) = l.split_once(" ") {
                return Some(val.parse::<i32>().unwrap());
            }
            None
        })
        .fold((1, 0), |acc: (i32, i32), item| {
            let mut register = acc.0;
            let mut cycle = acc.1;
            if let Some(val) = item {
                for i in 0..2 {
                    cycle += 1;
                    if cycle % WIDTH >= register && cycle % WIDTH < register + SPRITE_WIDTH {
                        let pixel = message.get_mut((cycle - 1) as usize).unwrap();
                        *pixel = "#";
                    }
                    if i == 1 {
                        register += val;
                    }
                }
            } else {
                cycle += 1;
                if cycle % WIDTH >= register && cycle % WIDTH < register + SPRITE_WIDTH {
                    let pixel = message.get_mut((cycle - 1) as usize).unwrap();
                    *pixel = "#";
                }
            }
            (register, cycle)
        });
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", message[(WIDTH * y + x) as usize]);
        }
        println!("");
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 10);
        assert_eq!(part_two(&input), None);
    }
}
