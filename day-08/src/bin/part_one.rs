use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

#[derive(Debug)]
struct PointPair {
    point_a: [isize; 2],
    point_b: [isize; 2],
}

impl PointPair {
    fn distance(self: &Self) -> [isize; 2] {
        let x_diff = self.point_a[0] - self.point_b[0];
        let y_diff = self.point_a[1] - self.point_b[1];

        [x_diff, y_diff]
    }

    fn antinodes(self: &Self, grid: &Vec<Vec<char>>) -> Vec<[isize; 2]> {
        let distance = self.distance();
        let mut antinodes: Vec<[isize; 2]> = vec![];

        if self.point_a[1] > self.point_b[1] {
            let antinode_a = [self.point_a[0] + distance[0], self.point_a[1] + distance[1]];
            let antinode_b = [self.point_b[0] - distance[0], self.point_b[1] - distance[1]];
            if on_grid(grid, antinode_a) {
                antinodes.push(antinode_a);
            }
            if on_grid(grid, antinode_b) {
                antinodes.push(antinode_b);
            }
        } else {
            let antinode_a = [self.point_a[0] - distance[0], self.point_a[1] - distance[1]];
            let antinode_b = [self.point_b[0] + distance[0], self.point_b[1] + distance[1]];
            if on_grid(grid, antinode_a) {
                antinodes.push(antinode_a);
            }
            if on_grid(grid, antinode_b) {
                antinodes.push(antinode_b);
            }
        }

        antinodes
    }
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antinode_coords: HashSet<[isize; 2]> = HashSet::new();
    let pairs = antenna_coordinates(&grid);

    for pair in pairs.iter() {
        let antinodes = pair.antinodes(&grid);
        for antinode in antinodes.iter() {
            antinode_coords.insert(*antinode);
        }
    }

    antinode_coords.len()
}

fn antenna_coordinates(grid: &Vec<Vec<char>>) -> Vec<PointPair> {
    let mut coords: HashMap<char, Vec<[isize; 2]>> = HashMap::new();
    let mut pairs: Vec<PointPair> = vec![];

    for y_idx in 0..grid.len() {
        for x_idx in 0..grid[0].len() {
            let grid_char = grid[y_idx][x_idx];

            if grid_char != '.' {
                coords
                    .entry(grid_char)
                    .or_insert_with(Vec::new)
                    .push([x_idx as isize, y_idx as isize]);
            }
        }
    }

    for value in coords.values() {
        let mut _pairs = generate_pairs(&mut value.clone());
        pairs.append(&mut _pairs);
    }

    pairs
}

fn generate_pairs(antenna: &mut Vec<[isize; 2]>) -> Vec<PointPair> {
    let mut pairs: Vec<PointPair> = vec![];

    while antenna.is_empty() == false {
        let first_el = antenna.pop().unwrap();
        for second_el in antenna.iter() {
            pairs.push(PointPair {
                point_a: first_el,
                point_b: *second_el,
            })
        }
    }

    pairs
}

fn on_grid(grid: &Vec<Vec<char>>, point: [isize; 2]) -> bool {
    if point[0] >= 0 && point[0] < grid[0].len() as isize {
        if point[1] >= 0 && point[1] < grid.len() as isize {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, 14);
    }
}
