use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(input);
    dbg!(output);
}

fn part_two(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let page_ordering_rules = iter.next().unwrap();
    let page_order = page_orders(page_ordering_rules);

    let updates: Vec<Vec<&str>> = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .collect();

    let mut sum = 0;

    for update in updates {
        let mut seen_page_numbers: HashSet<&str> = HashSet::new();

        for page_number in update.iter() {
            seen_page_numbers.insert(page_number);
            match page_order.get(page_number) {
                Some(set) => {
                    if let Some(_) = set.intersection(&seen_page_numbers).next() {
                        let ordered_update = order_update(&update, &page_order);
                        sum += ordered_update[ordered_update.len() / 2].parse::<usize>().unwrap();
                        break;
                    };
                }
                None => continue,
            }
        }
    }

    sum
}

fn page_orders(input_slice: &str) -> HashMap<&str, HashSet<&str>> {
    let mut page_orders: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input_slice.lines() {
        let pages = line.split('|').collect::<Vec<&str>>();

        page_orders
            .entry(pages[0])
            .or_insert_with(HashSet::new)
            .insert(pages[1]);
    }

    page_orders
}

fn order_update<'a>(update: &Vec<&'a str>, page_order: &HashMap<&str, HashSet<&str>>) -> Vec<&'a str> {
    let mut ordered_update: Vec<&str> = vec![];
    let mut seen_page_numbers: HashSet<&str> = HashSet::new();

    for page_number in update {
        seen_page_numbers.insert(page_number);
        match page_order.get(page_number) {
            Some(set) => {
                let seen_page_numbers_vec =
                    set.intersection(&seen_page_numbers).collect::<Vec<&&str>>();
                if seen_page_numbers_vec.len() == 0 {
                    ordered_update.push(page_number)
                } else {
                    let pos = seen_page_numbers_vec
                        .iter()
                        .map(|el| ordered_update.iter().position(|x| x == *el).unwrap())
                        .min()
                        .unwrap();

                    ordered_update.insert(pos, page_number);
                }
            }
            None => ordered_update.push(page_number),
        }
    }

    ordered_update
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
