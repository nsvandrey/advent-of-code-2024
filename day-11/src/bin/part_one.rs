use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let start = Instant::now();
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Result: {}\nDuration: {:?}", output, duration);
}

fn part_one(input: &str) -> usize {
    let mut stones: Vec<usize> = input.split(' ').map(|el| el.parse::<usize>().unwrap()).collect();
    
    for _ in 0..25 {
        blink(&mut stones);
    }

    stones.len()
}

fn blink(stones: &mut Vec<usize>) {
    let mut tmp_stones: Vec<usize> = vec![];

    for stone in &mut *stones {
        if *stone == 0 {
            tmp_stones.push(1);
        } else if stone.to_string().len() % 2 == 0  {
            let stone_string = stone.to_string();
            let (first, second) = stone_string.split_at(stone_string.len() / 2);
            tmp_stones.push(first.parse::<usize>().unwrap());
            tmp_stones.push(second.parse::<usize>().unwrap());
        } else {
             tmp_stones.push(*stone * 2024);
        }
    }
    
    *stones = tmp_stones;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, 55312);
    }
}