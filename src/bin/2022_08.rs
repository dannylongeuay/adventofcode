pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut grid: Vec<u8> = vec![0; height * width];
    for (y, row) in lines.iter().enumerate() {
        for (x, h) in row.chars().enumerate() {
            grid[width * y + x] = h.to_digit(10).unwrap() as u8;
        }
    }
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if is_visible(&grid, x, y, height, width) {
                sum += 1;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut grid: Vec<u8> = vec![0; height * width];
    for (y, row) in lines.iter().enumerate() {
        for (x, h) in row.chars().enumerate() {
            grid[width * y + x] = h.to_digit(10).unwrap() as u8;
        }
    }
    let mut max_score = 0;
    for y in 0..height {
        for x in 0..width {
            let score = scenic_score(&grid, x, y, height, width);
            if score > max_score {
                max_score = score;
            }
        }
    }
    Some(max_score)
}

fn is_visible(grid: &Vec<u8>, x: usize, y: usize, height: usize, width: usize) -> bool {
    let mut dy = 1;
    let h = grid[width * y + x];
    let mut top_vis = true;
    while y as isize - dy >= 0 {
        if grid[width * (y - dy as usize) + x] >= h {
            top_vis = false;
            break;
        }
        dy += 1;
    }
    dy = 1;
    let mut bot_vis = true;
    while y as isize + dy < height as isize {
        if grid[width * (y + dy as usize) + x] >= h {
            bot_vis = false;
            break;
        }
        dy += 1;
    }
    let mut dx = 1;
    let mut left_vis = true;
    while x as isize - dx >= 0 {
        if grid[width * y + (x - dx as usize)] >= h {
            left_vis = false;
            break;
        }
        dx += 1;
    }
    dx = 1;
    let mut right_vis = true;
    while x as isize + dx < width as isize {
        if grid[width * y + (x + dx as usize)] >= h {
            right_vis = false;
            break;
        }
        dx += 1;
    }
    return top_vis || bot_vis || left_vis || right_vis;
}

fn scenic_score(grid: &Vec<u8>, x: usize, y: usize, height: usize, width: usize) -> usize {
    let mut dy = 1;
    let h = grid[width * y + x];
    let mut top_score = 0;
    while y as isize - dy >= 0 {
        top_score += 1;
        if grid[width * (y - dy as usize) + x] >= h {
            break;
        }
        dy += 1;
    }
    dy = 1;
    let mut bot_score = 0;
    while y as isize + dy < height as isize {
        bot_score += 1;
        if grid[width * (y + dy as usize) + x] >= h {
            break;
        }
        dy += 1;
    }
    let mut dx = 1;
    let mut left_score = 0;
    while x as isize - dx >= 0 {
        left_score += 1;
        if grid[width * y + (x - dx as usize)] >= h {
            break;
        }
        dx += 1;
    }
    dx = 1;
    let mut right_score = 0;
    while x as isize + dx < width as isize {
        right_score += 1;
        if grid[width * y + (x + dx as usize)] >= h {
            break;
        }
        dx += 1;
    }
    return top_score * bot_score * left_score * right_score;
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2022, 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2022, 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2022, 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
