advent_of_code::solution!(4);

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>
}

impl Card {
    fn get_points(&self) -> u32 {
        let numbers_winning = self.drawn_numbers.iter()
            .filter(|drawn| self.winning_numbers.contains(drawn))
            .collect::<Vec<&u32>>();

        let power = numbers_winning.len() as u32;
        if power == 0 {
            return 0
        }

        2u32.pow(power - 1)
    }
}

fn parse_card(line: &str) -> Card {
    let [card, numbers] = (line.split(':').collect::<Vec<&str>>())[0..2] else {
        panic!("split did not work, invalid format on line {line}")
    };

    let id = card.strip_prefix("Card ")
        .unwrap_or("0")
        .parse::<u32>()
        .unwrap_or(0);

    let [winning_numbers_string, drawn_numbers_string] = (numbers.split('|').collect::<Vec<&str>>())[0..2] else {
        panic!("split did not work, invalid format on numbers {numbers}")
    };

    let winning_numbers = winning_numbers_string
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let drawn_numbers = drawn_numbers_string
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    Card { id, winning_numbers, drawn_numbers }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_card)
        .map(|card| card.get_points())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
