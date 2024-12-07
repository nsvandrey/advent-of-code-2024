use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_one(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

fn part_one(input: &str) -> isize {
    let result = input.lines().map(|line| parse_line(line)).sum();

    result
}

fn parse_line(line: &str) -> isize {
    let mut iter = line.split(": ");
    let target = iter.next().unwrap().parse::<isize>().unwrap();
    let mut nums: Vec<isize> = iter
        .next()
        .unwrap()
        .split(' ')
        .map(|el| el.parse::<isize>().unwrap())
        .collect();

    if is_valid_equation(&mut nums, target) {
        return target;
    }

    0
}

fn is_valid_equation(nums: &mut Vec<isize>, target: isize) -> bool {
    if nums.is_empty() {
        return target == 0;
    } else if target < 0 {
        return false;
    }

    let val = nums.pop().unwrap();

    if target % val == 0 {
        return is_valid_equation(&mut nums.clone(), target.clone() - val)
        || is_valid_equation(&mut nums.clone(), target.clone() / val)
    }

    is_valid_equation(&mut nums.clone(), target.clone() - val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
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
        assert_eq!(result, 3749);
    }
}
