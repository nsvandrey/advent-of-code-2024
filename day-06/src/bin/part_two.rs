use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_two(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
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
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut seen_positions: HashSet<[usize; 2]> = HashSet::new();
    let mut guard = Guard::new(&grid);
    let mut loops = 0;

    while guard.on_grid {
        let mut guard_clone = guard.clone();
        guard.move_guard(&grid);
        let pos = guard.position;

        match seen_positions.insert(pos) {
            true => grid[pos[1]][pos[0]] = '#',
            false => continue,
        }

        let mut clone_seen_positions: HashSet<(usize, usize, char)> = HashSet::new();

        while guard_clone.on_grid {
            match clone_seen_positions.insert((
                guard_clone.position[0],
                guard_clone.position[1],
                guard_clone.guard,
            )) {
                true => guard_clone.move_guard(&grid),
                false => {
                    loops += 1;
                    break;
                }
            }
        }

        grid[pos[1]][pos[0]] = '.';
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
