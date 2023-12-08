use std::{cmp::Ordering, collections::HashMap};

use crate::inputs::day7::PUZZLE_INPUT;

pub fn solution(part: u32, custom_solution: Option<&str>) -> u128 {
    let mut input = get_input(custom_solution, part);

    input.sort_by(|a, b| a.cmp(b, part));

    input
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum::<u32>() as u128
}

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
                .into()
        ),
        6440
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 250254244)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
                .into()
        ),
        5905
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 250087440)
}

#[derive(Debug)]
struct Hand {
    card_values: Vec<u32>,
    bid: u32,
}

#[derive(Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub fn cmp(&self, hand_type2: &HandType) -> Ordering {
        let value1 = *self as u32;
        let value2 = *hand_type2 as u32;

        value1.cmp(&value2)
    }
}

impl Hand {
    pub fn new(cards_str: &str, bid: u32, part: u32) -> Hand {
        Hand {
            card_values: cards_str
                .chars()
                .map(|character| match character {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if part == 1 {
                            11
                        } else {
                            0
                        }
                    }
                    'T' => 10,
                    _ => character.to_digit(10).unwrap(),
                })
                .collect(),
            bid,
        }
    }

    // returns handtype and number of jokers
    pub fn get_type(&self, part: u32) -> HandType {
        let mut card_nums = HashMap::<u32, u32>::new();

        for card in &self.card_values {
            if let Some(&old_num) = card_nums.get(card) {
                card_nums.insert(*card, old_num + 1);
            } else {
                card_nums.insert(*card, 1);
            }
        }

        let joker_value = if part == 1 { 11 } else { 0 };
        let num_jokers = *card_nums.get(&joker_value).unwrap_or(&0);
        let card_nums: Vec<u32> = card_nums.values().copied().collect();

        match card_nums.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_nums.contains(&4) {
                    // 4,1
                    if num_jokers > 0 && part == 2 {
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else {
                    // 3,2
                    if part == 2 && num_jokers > 0 {
                        if num_jokers == 1 {
                            HandType::FourOfAKind
                        } else {
                            HandType::FiveOfAKind
                        }
                    } else {
                        HandType::FullHouse
                    }
                }
            }
            3 => {
                if card_nums.contains(&3) {
                    // 3,1,1
                    if part == 2 && num_jokers > 0 {
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    // 2,2,1
                    if part == 2 && num_jokers > 0 {
                        if num_jokers == 1 {
                            HandType::FullHouse
                        } else {
                            HandType::FourOfAKind
                        }
                    } else {
                        HandType::TwoPair
                    }
                }
            }
            4 => {
                // 2,1,1,1
                if part == 2 && num_jokers > 0 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            _ => {
                // 1,1,1,1,1
                if part == 2 && num_jokers > 0 {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        }
    }

    pub fn cmp(&self, hand2: &Hand, part: u32) -> Ordering {
        match self.get_type(part).cmp(&hand2.get_type(part)) {
            Ordering::Equal => {
                for (i, &card_value1) in self.card_values.iter().enumerate() {
                    let card_value2 = hand2.card_values[i];

                    match card_value1.cmp(&card_value2) {
                        Ordering::Equal => (),
                        ord => return ord,
                    }
                }

                Ordering::Equal
            }
            ord => ord,
        }
    }
}

fn get_input(custom_solution: Option<&str>, part: u32) -> Vec<Hand> {
    let raw_input = custom_solution.unwrap_or(PUZZLE_INPUT);

    raw_input
        .lines()
        .map(|line| {
            Hand::new(
                line.split_once(' ').unwrap().0,
                line.split_once(' ').unwrap().1.parse::<u32>().unwrap(),
                part,
            )
        })
        .collect()
}
