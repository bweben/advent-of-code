advent_of_code::solution!(4);

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn get_points(&self) -> u32 {
        let power = self.get_amount_winning();
        if power == 0 {
            return 0;
        }

        2u32.pow(power - 1)
    }

    fn get_winning_cards(&self) -> Vec<u32> {
        let amount = self.get_amount_winning();
        let mut result: Vec<u32> = Vec::new();

        for i in 0..amount {
            result.push(self.id + i + 1);
        }

        result
    }

    fn get_amount_winning(&self) -> u32 {
        let numbers_winning = self.drawn_numbers.iter()
            .filter(|drawn| self.winning_numbers.contains(drawn))
            .collect::<Vec<&u32>>();

        numbers_winning.len() as u32
    }
}

fn parse_card(line: &str) -> Card {
    let [card, numbers] = (line.split(':').collect::<Vec<&str>>())[0..2] else {
        panic!("split did not work, invalid format on line {line}")
    };

    let id = card.strip_prefix("Card")
        .unwrap()
        .trim()
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
    let cards: Vec<Card> = input.lines()
        .filter(|line| !line.is_empty())
        .map(parse_card)
        .collect();
    let mut count = vec![1; cards.len()];

    for card in cards {
        for winning_card in card.get_winning_cards() {
            let card_count = (count[card.id as usize - 1] * 1) + count[winning_card as usize - 1];
            count[winning_card as usize - 1] = card_count;
        }
    }

    Some(count.iter().sum())
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
        assert_eq!(result, Some(30));
    }
}
