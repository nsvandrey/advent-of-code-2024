fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    input.lines().map(|line| parse_line(line)).sum()
}

fn parse_line(line: &str) -> usize {
    let report: Vec<isize> = line
        .split_whitespace()
        .map(|el| el.parse::<isize>().unwrap())
        .collect();
    
    let levels: Vec<isize> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    for window in levels.windows(2) {
        if window[0].abs() > 3 || window[0].abs() < 1 {
            return 0;
        }

        if window[1].abs() > 3 || window[1].abs() < 1 {
            return 0;
        }

        if window[0] * window[1] <= 0 {
            return 0;
        }
    }

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 4)
    }
}
