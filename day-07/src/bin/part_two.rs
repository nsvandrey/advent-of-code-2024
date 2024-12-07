use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_two(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

fn part_two(input: &str) -> usize {
    let result = input.lines().map(|line| parse_line(line)).sum();

    result
}

fn parse_line(line: &str) -> usize {
    let mut iter = line.split(": ");
    let target = iter.next().unwrap().parse::<usize>().unwrap();
    let mut nums: VecDeque<usize> = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|el| el.parse::<usize>().unwrap())
        .collect();

    let start_val = nums.pop_front().unwrap();

    if is_valid_equation(&mut nums, target, start_val) {
        return target;
    }

    0
}

fn concat(a: usize, b: usize) -> usize {
    let a_str = a.to_string();
    let b_str = b.to_string();

    let new_num = format!("{a_str}{b_str}");

    new_num.parse::<usize>().unwrap()
}

fn is_valid_equation(nums: &mut VecDeque<usize>, target: usize, current_value: usize) -> bool {
    if nums.is_empty() {
        return current_value == target;
    } else if current_value > target {
        return false;
    }
    let val = nums.pop_front().unwrap();

    is_valid_equation(&mut nums.clone(), target, current_value.clone() + val)
        || is_valid_equation(&mut nums.clone(), target, current_value.clone() * val)
        || is_valid_equation(
            &mut nums.clone(),
            target,
            concat(current_value.clone(), val),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 11387);
    }
}
