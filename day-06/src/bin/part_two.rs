use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(input);
    dbg!(output);
}

#[derive(Clone)]
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

fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let guard = Guard::new(&grid);

    let mut loops = 0;

    for y_idx in 0..grid.len() {
        for x_idx in 0..grid[0].len() {
            let mut grid_clone = grid.clone();

            if grid_clone[y_idx][x_idx] == '#' || grid_clone[y_idx][x_idx] == '^' {
                continue;
            }

            grid_clone[y_idx][x_idx] = '#';

            let mut guard_clone = guard.clone();
            let mut seen_positions: HashSet<(usize, usize, char)> = HashSet::new();

            while guard_clone.on_grid {
                match seen_positions.insert((
                    guard_clone.position[0],
                    guard_clone.position[1],
                    guard_clone.guard,
                )) {
                    true => guard_clone.move_guard(&grid_clone),
                    false => {
                        loops += 1;
                        break;
                    }
                }
            }
        }
    }

    loops
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
    fn test_part_two() {
        let result = part_two(
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
        assert_eq!(result, 6);
    }
}
