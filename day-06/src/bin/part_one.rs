use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

struct Guard {
    guard: char,
    position: [usize; 2],
    on_grid: bool,
}

impl Guard {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        Self {
            guard: '^',
            position: find_guard_start(grid).unwrap(),
            on_grid: true,
        }
    }

    fn move_guard(self: &mut Self, grid: &Vec<Vec<char>>) {
        match self.guard {
            '^' => {
                if self.position[1] == 0 {
                    self.on_grid = false;
                } else if grid[self.position[1] - 1][self.position[0]] == '#' {
                    self.guard = '>';
                } else {
                    self.position = [self.position[0], self.position[1] - 1]
                }
            }
            '>' => {
                if self.position[0] == grid[0].len() - 1 {
                    self.on_grid = false;
                } else if grid[self.position[1]][self.position[0] + 1] == '#' {
                    self.guard = 'v';
                } else {
                    self.position = [self.position[0] + 1, self.position[1]]
                }
            }
            'v' => {
                if self.position[1] == grid.len() - 1 {
                    self.on_grid = false;
                } else if grid[self.position[1] + 1][self.position[0]] == '#' {
                    self.guard = '<';
                } else {
                    self.position = [self.position[0], self.position[1] + 1]
                }
            }
            _ => {
                if self.position[0] == 0 {
                    self.on_grid = false;
                } else if grid[self.position[1]][self.position[0] - 1] == '#' {
                    self.guard = '^';
                } else {
                    self.position = [self.position[0] - 1, self.position[1]]
                }
            }
        }
    }
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_positions: HashSet<[usize; 2]> = HashSet::new();
    let mut guard = Guard::new(&grid);

    while guard.on_grid {
        seen_positions.insert(guard.position);
        guard.move_guard(&grid);
    }

    seen_positions.len()
}

fn find_guard_start(grid: &Vec<Vec<char>>) -> Option<[usize; 2]> {
    let mut guard_pos = None;
    for y_idx in 0..grid.len() {
        for x_idx in 0..grid[0].len() {
            if grid[y_idx][x_idx] == '^' {
                guard_pos = Some([x_idx, y_idx]);
                break;
            }
        }
    }

    guard_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );
        assert_eq!(result, 41);
    }
}
