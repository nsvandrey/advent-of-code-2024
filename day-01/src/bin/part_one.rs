fn main() {
    let input = include_str!("./input.txt");
    let output = part_one(&input);
    dbg!(output);
}

fn part_one(input: &str) -> usize {
    let mut first_list: Vec<usize> = vec![];
    let mut second_list: Vec<usize> = vec![];

    for line in input.lines() {
        let mut pair = line.split_whitespace();
        first_list.push(pair.next().unwrap().parse::<usize>().unwrap());
        second_list.push(pair.next().unwrap().parse::<usize>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let distance = first_list
        .iter()
        .zip(second_list.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum();

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 11)
    }
}
