fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(input);
    dbg!(output);
}

enum Direction {
    NW,
    NE,
    SE,
    SW,
}

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    for y_idx in 1..(grid.len() - 1) {
        for x_idx in 1..(grid[0].len() - 1) {
            let current_point = [x_idx, y_idx];
            if grid[y_idx][x_idx] == 'A' {
                if check_corners(&grid, current_point) {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn traverse_grid(grid: &Vec<Vec<char>>, current_point: [usize; 2], direction: &Direction) -> char {
    match direction {
        Direction::NW => grid[current_point[1] - 1][current_point[0] - 1].clone(),
        Direction::NE => grid[current_point[1] - 1][current_point[0] + 1].clone(),
        Direction::SE => grid[current_point[1] + 1][current_point[0] + 1].clone(),
        Direction::SW => grid[current_point[1] + 1][current_point[0] - 1].clone(),
    }
}

fn check_corners(grid: &Vec<Vec<char>>, point: [usize; 2]) -> bool {
    let mut corners: Vec<char> = vec![];

    for direction in [Direction::NW, Direction::NE, Direction::SE, Direction::SW].iter() {
        let c = traverse_grid(grid, point, direction);

        if c == 'X' || c == 'A' {
            return false;
        }

        corners.push(c);
    }

    if corners[0] == corners[1] && corners[2] == corners[3] {
        return corners[0] != corners[2];
    }

    if corners[0] == corners[3] && corners[1] == corners[2] {
        return corners[0] != corners[1];
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 9);
    }
}
