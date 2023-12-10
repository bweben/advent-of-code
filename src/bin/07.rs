use std::cmp::Ordering;
use std::collections::HashMap;


use crate::Type::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};

advent_of_code::solution!(7);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl Type {
    fn new(cards: Vec<u8>, with_joker: bool) -> Self {
        let mut map: HashMap<u8, u8> = HashMap::new();
        for card in cards {
            let value = *map.get(&card).unwrap_or(&0);
            map.insert(card, value + 1);
        }

        let joker_set = *map.get(&11).unwrap_or(&0);

        if map.len() == 1 {
            return FiveOfKind;
        }

        if map.len() == 2 && *(map.values().max().unwrap()) == 4 {
            if with_joker && (joker_set == 1 || joker_set == 4) {
                return FiveOfKind;
            }

            return FourOfKind;
        }

        if map.len() == 2 {
            if with_joker && (joker_set == 2 || joker_set == 3) {
                return FiveOfKind;
            }

            return FullHouse;
        }

        if map.len() == 3 && *(map.values().max().unwrap()) == 3 {
            if with_joker && (joker_set == 1 || joker_set == 3) {
                return FourOfKind;
            }

            return ThreeOfKind;
        }

        if map.len() == 3 {
            if with_joker {
                if joker_set == 2 {
                    return FourOfKind;
                }

                if joker_set == 1 {
                    return FullHouse;
                }
            }

            return TwoPair;
        }

        if *map.values().max().unwrap() == 2 {
            if with_joker && (joker_set == 1 || joker_set == 2) {
                return ThreeOfKind;
            }

            return OnePair;
        }

        if with_joker && joker_set == 1 {
            return OnePair
        }

        HighCard
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<u8>,
    // T = 10, J = 11, Q = 12, K = 13, A = 14
    bid: u32,
    card_type: Type,
    with_joker: bool,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.card_type == other.card_type {
            if self.cards.len() != other.cards.len() {
                return None;
            }

            for (i, card) in self.cards.iter().enumerate() {
                let other_card = *other.cards.get(i).unwrap();

                if *card == other_card {
                    continue;
                }

                if self.with_joker && *card == 11 {
                    return Some(Ordering::Less);
                }

                if other.with_joker && other_card == 11 {
                    return Some(Ordering::Greater);
                }

                if *card > other_card {
                    return Some(Ordering::Greater);
                } else {
                    return Some(Ordering::Less);
                }
            }

            return Some(Ordering::Equal);
        }

        self.card_type.partial_cmp(&other.card_type)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn handle_cards(input: &str, with_joker: bool) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines()
        .map(|line| line.split_ascii_whitespace())
        .map(|mut line| {
            let cards = line.next();
            let bid = line.next();

            let mut converted_cards: Vec<u8> = Vec::new();

            for card in cards.unwrap().chars() {
                if card.is_ascii_digit() {
                    converted_cards.push(card.to_digit(10).unwrap() as u8);
                } else {
                    converted_cards.push(match card {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        _ => 14 // A
                    })
                }
            }

            Hand {
                cards: converted_cards.clone(),
                bid: bid.unwrap().parse::<u32>().unwrap(),
                card_type: Type::new(converted_cards, with_joker),
                with_joker,
            }
        }).collect();

    hands.sort();

    let result = hands.iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    handle_cards(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    handle_cards(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(5905));
    }
}
