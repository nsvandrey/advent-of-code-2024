use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Result: {}\nDuration: {:?}", output, duration);
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;

    for y_idx in 0..grid.len() {
        for x_idx in 0..grid[0].len() {
            if grid[y_idx][x_idx] == '0' {
                let mut seen_points: HashSet<[usize; 2]> = HashSet::new();
                let mut frontier: Vec<[usize; 2]> = vec![];
                frontier.push([x_idx, y_idx]);
                while frontier.len() > 0 {
                    let point = frontier.pop().unwrap();
                    if grid[point[1]][point[0]] == '9' {
                        result += 1;
                    } else {
                        check_neighbors(point, &grid, &mut seen_points, &mut frontier);
                    }
                }
            }
        }
    }

    result
}

fn check_neighbors(
    point: [usize; 2],
    grid: &Vec<Vec<char>>,
    seen_points: &mut HashSet<[usize; 2]>,
    frontier: &mut Vec<[usize; 2]>,
) {
    let target_val = grid[point[1]][point[0]].to_digit(10).unwrap() + 1;
    let mut valid_neighbors: Vec<[usize; 2]> = vec![];

    if point[0] > 0 {
        valid_neighbors.push([point[0] - 1, point[1]]);
    }
    if point[0] < grid[0].len() - 1 {
        valid_neighbors.push([point[0] + 1, point[1]]);
    }
    if point[1] > 0 {
        valid_neighbors.push([point[0], point[1] - 1]);
    }
    if point[1] < grid.len() - 1 {
        valid_neighbors.push([point[0], point[1] + 1]);
    }

    for neighbor in valid_neighbors {
        let target = grid[neighbor[1]][neighbor[0]].to_digit(10).unwrap();
        if target == target_val {
            match seen_points.insert(neighbor) {
                true => {
                    frontier.push(neighbor);
                }
                false => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, 36);
    }
}
