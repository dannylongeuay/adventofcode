use anyhow::Error;
use std::str::FromStr;

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_condition: usize,
    true_throw: usize,
    false_throw: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: Vec::new(),
            operation: Operation {
                a: "".to_string(),
                b: "".to_string(),
                operand: Operand::ADD,
            },
            test_condition: 0,
            true_throw: 0,
            false_throw: 0,
        }
    }
}

#[derive(Debug)]
enum Operand {
    ADD,
    MULTIPLY,
}

#[derive(Debug)]
struct Operation {
    a: String,
    b: String,
    operand: Operand,
}

impl Operation {
    fn run(&self, item: usize) -> usize {
        let aa = match self.a.as_str() {
            "old" => item,
            _ => self.a.parse::<usize>().unwrap(),
        };
        let bb = match self.b.as_str() {
            "old" => item,
            _ => self.b.parse::<usize>().unwrap(),
        };
        match self.operand {
            Operand::ADD => aa + bb,
            Operand::MULTIPLY => aa * bb,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.trim().split(" ").collect();
        Ok(Operation {
            a: words[2].to_string(),
            b: words[4].to_string(),
            operand: match words[3] {
                "+" => Operand::ADD,
                "*" => Operand::MULTIPLY,
                _ => unreachable!("bad input for operand"),
            },
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = get_monkeys(input);
    Some(get_inspects(&mut monkeys, 20, true))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = get_monkeys(input);
    Some(get_inspects(&mut monkeys, 10000, false))
}

fn get_inspects(monkeys: &mut Vec<Monkey>, rounds: usize, should_divde: bool) -> usize {
    let mut inspects: Vec<usize> = vec![0; monkeys.len()];
    let monkey_lcm = monkeys.iter().map(|m| m.test_condition).product::<usize>();
    for _round in 1..=rounds {
        for j in 0..monkeys.len() {
            let monkey_thrower = monkeys.get_mut(j).unwrap();
            let true_throw = monkey_thrower.true_throw;
            let false_throw = monkey_thrower.false_throw;
            let mut true_throws: Vec<usize> = Vec::new();
            let mut false_throws: Vec<usize> = Vec::new();
            while let Some(old_item) = monkey_thrower.items.pop() {
                inspects[j] += 1;
                let mut new_item = monkey_thrower.operation.run(old_item);
                if should_divde {
                    new_item /= 3;
                } else {
                    new_item %= monkey_lcm;
                }
                if new_item % monkey_thrower.test_condition == 0 {
                    true_throws.push(new_item);
                } else {
                    false_throws.push(new_item);
                }
            }
            let monkey_catcher_true = monkeys.get_mut(true_throw).unwrap();
            monkey_catcher_true.items.append(&mut true_throws);
            let monkey_catcher_false = monkeys.get_mut(false_throw).unwrap();
            monkey_catcher_false.items.append(&mut false_throws);
        }
    }
    inspects.sort();
    inspects.iter().rev().take(2).product::<usize>()
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let monkey_stats: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for stats in monkey_stats.iter() {
        let lines = stats.lines();
        let mut monkey = Monkey::new();
        for line in lines {
            let words: Vec<&str> = line.trim().split(":").collect();
            match words[0] {
                "Starting items" => {
                    monkey.items = words[1]
                        .trim()
                        .split(", ")
                        .map(|i| i.parse::<usize>().unwrap())
                        .collect();
                }
                "Operation" => {
                    monkey.operation = Operation::from_str(words[1]).unwrap();
                }
                "Test" => {
                    monkey.test_condition = words[1]
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                "If true" => {
                    monkey.true_throw = words[1]
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                "If false" => {
                    monkey.false_throw = words[1]
                        .split(" ")
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                _ => {}
            }
        }
        monkeys.push(monkey);
    }
    monkeys
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
