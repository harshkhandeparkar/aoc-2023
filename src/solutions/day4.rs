use std::collections::HashSet;

use crate::inputs::day4::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u128 {
    let input = get_input(custom_input);
    let card_matching_nums: Vec<u128> = input
        .lines()
        .map(|line| {
            let (winning_cards, our_cards) = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_once('|')
                .unwrap();

            let winning_cards: HashSet<u128> = winning_cards
                .split_whitespace()
                .map(|number| number.trim().parse::<u128>().unwrap())
                .collect();

            let our_cards: HashSet<u128> = our_cards
                .split_whitespace()
                .map(|number| number.trim().parse::<u128>().unwrap())
                .collect();

            let num_cards_won = winning_cards
                .intersection(&our_cards)
                .collect::<HashSet<_>>()
                .len();

            num_cards_won as u128
        })
        .collect();

    if part == 1 {
        card_matching_nums
            .iter()
            .map(|num_won| {
                if *num_won > 0 {
                    2_u128.pow(*num_won as u32 - 1)
                } else {
                    0
                }
            })
            .sum::<u128>()
    } else {
        let mut num_cards: Vec<u128> = vec![1; card_matching_nums.len()];

        for (i, card_value) in card_matching_nums.iter().enumerate() {
            let num_card = num_cards[i];

            for j in i..(i + *card_value as usize) {
                if j + 1 < num_cards.len() {
                    num_cards[j + 1] += num_card;
                }
            }
        }

        num_cards.iter().sum::<u128>()
    }
}

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .into()
        ),
        13
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 25004)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .into()
        ),
        30
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 14427616)
}

fn get_input(custom_input: Option<&str>) -> String {
    String::from(custom_input.unwrap_or(PUZZLE_INPUT))
}
