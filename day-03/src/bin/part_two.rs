fn main() {
    let input = include_str!("./input.txt");
    let output = part_two(&input);
    dbg!(output);
}

#[derive(PartialEq)]
enum ParserState {
    Enabled,
    Disabled,
}

struct Parser {
    state: ParserState,
    instructions: usize,
}

impl Parser {
    fn new() -> Self {
        Self {
            state: ParserState::Enabled,
            instructions: 0,
        }
    }

    fn cycle_state(self: &mut Self) {
        if self.state == ParserState::Enabled {
            self.state = ParserState::Disabled;
        } else {
            self.state = ParserState::Enabled;
        }
    }

    fn process_instruction(self: &mut Self, instruction: &str) {
        let substring = &instruction[4..]
            .chars()
            .into_iter()
            .take_while(|c| *c != ')')
            .collect::<String>();

        let mul_elements = substring.split(",").collect::<Vec<&str>>();

        if mul_elements.len() != 2 {
            return;
        }

        let mul_numbers: Vec<usize> = mul_elements.iter().filter_map(|c| c.parse::<usize>().ok()).collect();

        if mul_numbers.len() != 2 {
            return;
        }

        self.instructions += mul_numbers[0] * mul_numbers[1];
    }

    fn process_memory(self: &mut Self, line: &str) {
        for idx in 0..line.len() {
            let substring = &line[idx..];

            if self.state == ParserState::Disabled {
                if substring.starts_with("do()") {
                    self.cycle_state();
                    continue;
                }
            } else {
                if substring.starts_with("don't()") {
                    self.cycle_state();
                    continue;
                }

                if substring.starts_with("mul(") {
                    self.process_instruction(substring);
                }
            }
        }
    }
}

fn part_two(input: &str) -> usize {
    let lines = input.lines();
    let mut parser = Parser::new();

    for line in lines {
        parser.process_memory(&line);
    }

    parser.instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}
