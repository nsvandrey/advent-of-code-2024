fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

struct TraverseResult {
    grid_char: char,
    point: [usize; 2],
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;

    for y_idx in 0..grid.len() {
        for x_idx in 0..grid[0].len() {
            if grid[y_idx][x_idx] == 'X' {
                let current_point = [x_idx, y_idx];
                for direction in get_valid_directions(&grid, current_point) {
                    let mut point = current_point;
                    let mut success = true;
                    for target in ['M', 'A', 'S'] {
                        let tr = traverse_grid(&grid, point, &direction);
                        if tr.grid_char != target {
                            success = false;
                            break;
                        }
                        point = tr.point;
                    }
                    if success {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn get_valid_directions(grid: &Vec<Vec<char>>, point: [usize; 2]) -> Vec<Direction> {
    let mut valid_directions: Vec<Direction> = vec![];

    if point[1] >= 3 {
        valid_directions.push(Direction::N);
    }

    if grid.len() - point[1] > 3 {
        valid_directions.push(Direction::S);
    }

    if grid[0].len() - point[0] > 3 {
        valid_directions.push(Direction::E);
        if point[1] >= 3 {
            valid_directions.push(Direction::NE);
        }
        if grid.len() - point[1] > 3 {
            valid_directions.push(Direction::SE);
        }
    }

    if point[0] >= 3 {
        valid_directions.push(Direction::W);
        if point[1] >= 3 {
            valid_directions.push(Direction::NW);
        }
        if grid.len() - point[1] > 3 {
            valid_directions.push(Direction::SW);
        }
    }

    valid_directions
}

fn traverse_grid(
    grid: &Vec<Vec<char>>,
    current_point: [usize; 2],
    direction: &Direction,
) -> TraverseResult {
    match direction {
        Direction::NW => TraverseResult {
            grid_char: (grid[current_point[1] - 1][current_point[0] - 1].clone()),
            point: [current_point[0] - 1, current_point[1] - 1],
        },
        Direction::N => TraverseResult {
            grid_char: grid[current_point[1] - 1][current_point[0]].clone(),
            point: [current_point[0], current_point[1] - 1],
        },
        Direction::NE => TraverseResult {
            grid_char: grid[current_point[1] - 1][current_point[0] + 1].clone(),
            point: [current_point[0] + 1, current_point[1] - 1],
        },
        Direction::E => TraverseResult {
            grid_char: grid[current_point[1]][current_point[0] + 1].clone(),
            point: [current_point[0] + 1, current_point[1]],
        },
        Direction::SE => TraverseResult {
            grid_char: grid[current_point[1] + 1][current_point[0] + 1].clone(),
            point: [current_point[0] + 1, current_point[1] + 1],
        },
        Direction::S => TraverseResult {
            grid_char: grid[current_point[1] + 1][current_point[0]].clone(),
            point: [current_point[0], current_point[1] + 1],
        },
        Direction::SW => TraverseResult {
            grid_char: grid[current_point[1] + 1][current_point[0] - 1].clone(),
            point: [current_point[0] - 1, current_point[1] + 1],
        },
        Direction::W => TraverseResult {
            grid_char: grid[current_point[1]][current_point[0] - 1].clone(),
            point: [current_point[0] - 1, current_point[1]],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
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
        assert_eq!(result, 18);
    }
}
