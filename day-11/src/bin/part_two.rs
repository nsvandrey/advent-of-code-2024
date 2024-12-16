use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part_two(input);
    let duration = start.elapsed();
    println!("Result: {}\nDuration: {:?}", output, duration);
}

fn part_two(input: &str) -> usize {
    let stone_vec: Vec<usize> = input
        .split(' ')
        .map(|el| el.parse::<usize>().unwrap())
        .collect();
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for stone in stone_vec {
        stones
            .entry(stone)
            .and_modify(|el| *el += 1)
            .or_insert(1);
    }

    for _ in 0..75 {
        blink(&mut stones);
    }

    stones.values().sum()
}

fn blink(stones: &mut HashMap<usize, usize>) {
    let mut tmp_stones: HashMap<usize, usize> = HashMap::new();

    for (stone, num) in &mut *stones {
        if *stone == 0 {
            tmp_stones
                .entry(1)
                .and_modify(|el| *el += *num)
                .or_insert(*num);
        } else if stone.to_string().len() % 2 == 0 {
            let stone_string = stone.to_string();
            let (first, second) = stone_string.split_at(stone_string.len() / 2);
            tmp_stones
                .entry(first.parse::<usize>().unwrap())
                .and_modify(|el| *el += *num)
                .or_insert(*num);
            tmp_stones
                .entry(second.parse::<usize>().unwrap())
                .and_modify(|el| *el += *num)
                .or_insert(*num);
        } else {
            tmp_stones
                .entry(stone * 2024)
                .and_modify(|el| *el += *num)
                .or_insert(*num);
        }
    }

    *stones = tmp_stones;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two("125 17");
        assert_eq!(result, 55312);
    }
}
