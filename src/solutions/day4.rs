use std::collections::HashSet;

use crate::inputs::day4::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> i128 {
    let input = get_input(custom_input);
    let card_matching_nums: Vec<i128> = input
        .lines()
        .map(|line| {
            let (winning_cards, our_cards) = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split_once('|')
                .unwrap();

            let winning_cards: HashSet<i128> = winning_cards
                .split_whitespace()
                .map(|number| number.trim().parse::<i128>().unwrap())
                .collect();

            let our_cards: HashSet<i128> = our_cards
                .split_whitespace()
                .map(|number| number.trim().parse::<i128>().unwrap())
                .collect();

            let num_cards_won = winning_cards
                .intersection(&our_cards)
                .collect::<HashSet<_>>()
                .len();

            num_cards_won as i128
        })
        .collect();

    if part == 1 {
        card_matching_nums
            .iter()
            .map(|num_won| {
                if *num_won > 0 {
                    2_i128.pow(*num_won as u32 - 1)
                } else {
                    0
                }
            })
            .sum::<i128>()
    } else {
        let mut num_cards: Vec<i128> = vec![1; card_matching_nums.len()];

        for (i, card_value) in card_matching_nums.iter().enumerate() {
            let num_card = num_cards[i];

            for j in i..(i + *card_value as usize) {
                if j + 1 < num_cards.len() {
                    num_cards[j + 1] += num_card;
                }
            }
        }

        num_cards.iter().sum::<i128>()
    }
}

fn get_input(custom_input: Option<&str>) -> String {
    String::from(custom_input.unwrap_or(PUZZLE_INPUT))
}
