pub fn part_one(input: &str) -> Option<i32> {
    let (h_sum, v_sum) = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let dir = parts[0];
            let dist = parts[1].parse::<i32>().unwrap();
            let mut h: i32 = 0;
            let mut v: i32 = 0;
            match dir {
                "forward" => h += dist,
                "down" => v += dist,
                "up" => v -= dist,
                _ => unreachable!(),
            };
            (h, v)
        })
        .fold((0, 0), |accum, item| (accum.0 + item.0, accum.1 + item.1));
    Some(h_sum * v_sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut a: i32 = 0;
    let (h_sum, v_sum) = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let dir = parts[0];
            let dist = parts[1].parse::<i32>().unwrap();
            let mut h: i32 = 0;
            let mut v: i32 = 0;
            match dir {
                "forward" => {
                    h += dist;
                    v += a * dist
                }
                "down" => a += dist,
                "up" => a -= dist,
                _ => unreachable!(),
            };
            (h, v)
        })
        .fold((0, 0), |accum, item| (accum.0 + item.0, accum.1 + item.1));
    Some(h_sum * v_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2021, 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2021, 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2021, 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
