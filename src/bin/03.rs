advent_of_code::solution!(3);

struct Digit {
    line_number: usize,
    start_index: usize,
    end_index: usize,
    value: u32,
}

struct Symbol {
    line_number: usize,
    index: usize,
    value: char,
}

impl Digit {
    fn is_symbol_near(&self, symbols: &Vec<Symbol>) -> bool {
        for x in symbols {
            if self.is_given_symbol_near(x) {
                return true;
            }
        }

        false
    }

    fn is_given_symbol_near(&self, x: &Symbol) -> bool {
        if x.line_number == self.line_number && (x.index == self.end_index + 1 || x.index == self.start_index.saturating_sub(1)) {
            return true;
        }

        if (x.line_number == self.line_number.saturating_sub(1) || x.line_number == self.line_number + 1) && x.index <= self.end_index + 1 && x.index >= self.start_index.saturating_sub(1) {
            return true;
        }

        return false;
    }
}

impl Symbol {
    fn get_gear_ratio(&self, digits: &Vec<Digit>) -> Option<u32> {
        let filtered_digits = digits.iter()
            .filter(|digit| digit.is_given_symbol_near(self))
            .collect::<Vec<&Digit>>();
        if filtered_digits.len() < 2 {
            return None;
        }

        let [first_digit, second_digit] = (filtered_digits)[0..2] else {
            return None;
        };

        Some(first_digit.value * second_digit.value)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut digits: Vec<Digit> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (line_index, line) in input.lines()
        .filter(|line| !line.is_empty())
        .enumerate() {
        parse_line(&mut digits, &mut symbols, line_index, line);
    };

    let result = digits.iter()
        .filter(|digit| digit.is_symbol_near(&symbols))
        .map(|digit| digit.value)
        .sum();

    Some(result)
}

fn parse_line(digits: &mut Vec<Digit>, symbols: &mut Vec<Symbol>, line_index: usize, line: &str) {
    let mut digit = String::new();
    let mut start_index = usize::MAX;
    for (i, x) in line.chars().enumerate() {
        if x.is_ascii_digit() {
            if start_index == usize::MAX {
                start_index = i;
                digit = String::new();
            }

            digit += &x.to_string();
            continue;
        }

        if start_index != usize::MAX {
            digits.push(Digit {
                line_number: line_index,
                start_index,
                end_index: i - 1,
                value: digit.parse::<u32>().unwrap_or(0),
            });
            start_index = usize::MAX;
        }

        if x != '.' {
            symbols.push(Symbol {
                line_number: line_index,
                index: i,
                value: x,
            });
        }
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut digits: Vec<Digit> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (line_index, line) in input.lines()
        .filter(|line| !line.is_empty())
        .enumerate() {
        parse_line(&mut digits, &mut symbols, line_index, line);
    };

    let result = symbols.iter()
        .filter(|symbol| symbol.value == '*')
        .filter_map(|symbol| symbol.get_gear_ratio(&digits))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(467835));
    }
}
