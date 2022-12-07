use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let dir_sizes = calc_dir_sizes(input);

    Some(dir_sizes.into_values().fold(0, |acc, size| {
        if size <= 100_000 {
            return acc + size;
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let dir_sizes = calc_dir_sizes(input);

    let unused_space = 70_000_000 - dir_sizes.get("/").unwrap();

    let mut smallest = usize::MAX;
    for (_k, v) in dir_sizes.iter() {
        if *v + unused_space < 30_000_000 {
            continue;
        }
        if *v < smallest {
            smallest = *v;
        }
    }
    Some(smallest)
}

fn calc_dir_sizes(input: &str) -> HashMap<String, usize> {
    let lines = input.lines();
    let mut parents: Vec<&str> = vec!["/"];
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for line in lines {
        let (first, second) = line.split_once(" ").unwrap();
        match first {
            "$" => {
                if let Some((cmd, arg)) = second.split_once(" ") {
                    assert!(cmd == "cd");
                    match arg {
                        "/" => {
                            parents = vec!["/"];
                        }
                        ".." => {
                            parents.pop();
                        }
                        _ => {
                            parents.push(arg);
                        }
                    }
                }
            }
            "dir" => {}
            _ => {
                let size = first.parse::<usize>().unwrap();
                for i in 0..parents.len() {
                    let dir_key = parents[..i + 1].join("_");
                    if !dir_sizes.contains_key(&dir_key) {
                        dir_sizes.insert(dir_key.clone(), 0);
                    }
                    let dir_size = dir_sizes.get_mut(&dir_key).unwrap();
                    *dir_size += size;
                }
            }
        }
    }
    dir_sizes
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
