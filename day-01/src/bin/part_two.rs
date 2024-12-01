use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(&input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let mut first_list: Vec<usize> = vec![];
    let mut second_list: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut pair = line.split_whitespace();
        first_list.push(pair.next().unwrap().parse::<usize>().unwrap());

        let second_element = pair.next().unwrap().parse::<usize>().unwrap();
        second_list
            .entry(second_element)
            .and_modify(|el| *el += 1)
            .or_insert(1);
    }

    let mut similiarity = 0;

    for el in first_list {
        match second_list.get(&el) {
            Some(num) => similiarity += &el * num,
            None => continue,
        }
    }

    similiarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 31)
    }
}
