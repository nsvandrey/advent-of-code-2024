use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = include_str!("./input.txt");
    let output = part_two(input);
    let duration = start.elapsed();
    println!("Output: {}\nDuration: {:?}", output, duration);
}

fn part_two(input: &str) -> isize {
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

fn concat(target: &isize, val: &isize) -> isize {
    let target_str = target.to_string();
    let val_str = val.to_string();

    let  new_num = target_str.chars().rev().skip(val_str.len()).collect::<String>();
    let new_num = new_num.chars().rev().collect::<String>();

    new_num.parse::<isize>().unwrap()
}

fn is_valid_equation(nums: &mut Vec<isize>, target: isize) -> bool {
    if nums.is_empty() {
        return target == 0;
    } else if target < 0 {
        return false;
    }

    let val = nums.pop().unwrap();

    let should_multiply = target % val == 0;
    let should_concat = target.to_string().ends_with(&val.to_string());

    if should_multiply {
        if should_concat {
            return is_valid_equation(&mut nums.clone(), target.clone() - val)
                || is_valid_equation(&mut nums.clone(), target.clone() / val)
                || is_valid_equation(&mut nums.clone(), concat(&target.clone(), &val));
        }
        return is_valid_equation(&mut nums.clone(), target.clone() - val)
            || is_valid_equation(&mut nums.clone(), target.clone() / val);
    } else if should_concat {
        return is_valid_equation(&mut nums.clone(), target.clone() - val)
            || is_valid_equation(&mut nums.clone(), concat(&target.clone(), &val));
    }

    is_valid_equation(&mut nums.clone(), target.clone() - val)
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
