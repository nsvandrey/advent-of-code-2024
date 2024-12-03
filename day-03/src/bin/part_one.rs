fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let lines = input.lines().map(|line| parse_line(line)).sum();
    lines
}

fn parse_line(line: &str) -> usize {
    let instructions: usize = line
        .split("mul(")
        .map(|el| el.split(")").collect::<Vec<&str>>())
        .filter(|vec| vec.len() > 1)
        .map(|vec| {
            vec[0]
                .split(",")
                .filter_map(|c| c.parse().ok())
                .collect::<Vec<usize>>()
        })
        .filter(|vec| vec.len() == 2)
        .map(|vec| vec[0] * vec[1])
        .sum();

    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}
