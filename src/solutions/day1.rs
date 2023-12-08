//! https://adventofcode.com/2023/day/1

use crate::inputs::day1::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u32 {
    if part == 1 {
        get_input(custom_input)
            .lines()
            .map(|line| {
                let line_digits: Vec<u32> = line
                    .chars()
                    .map(|c| c.to_digit(10))
                    .filter(|o| o.is_some())
                    .flatten()
                    .collect();

                10 * line_digits.first().unwrap() + line_digits.last().unwrap()
            })
            .sum::<u32>()
    } else {
        let input = get_input(custom_input);
        let words_digit_map: [(u32, &str); 18] = [
            (1, "1"),
            (2, "2"),
            (3, "3"),
            (4, "4"),
            (5, "5"),
            (6, "6"),
            (7, "7"),
            (8, "8"),
            (9, "9"),
            (1, "one"),
            (2, "two"),
            (3, "three"),
            (4, "four"),
            (5, "five"),
            (6, "six"),
            (7, "seven"),
            (8, "eight"),
            (9, "nine"),
        ];

        input
            .lines()
            .map(|line| {
                let mut digit_occurences: Vec<(&u32, usize)> = words_digit_map
                    .iter()
                    .map(|(digit, word)| (digit, line.find(word)))
                    .filter(|(_, loc)| loc.is_some())
                    .map(|(digit, loc)| (digit, loc.unwrap()))
                    .collect();
                digit_occurences.sort_by(|a, b| a.1.cmp(&b.1));
                let first_digit = digit_occurences.first().unwrap().0;

                let mut digit_occurences: Vec<(&u32, usize)> = words_digit_map
                    .iter()
                    .map(|(digit, word)| (digit, line.rfind(word)))
                    .filter(|(_, loc)| loc.is_some())
                    .map(|(digit, loc)| (digit, loc.unwrap()))
                    .collect();
                digit_occurences.sort_by(|a, b| a.1.cmp(&b.1));
                let last_digit = digit_occurences.last().unwrap().0;

                10 * first_digit + last_digit
            })
            .sum::<u32>()
    }
}

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
                .into()
        ),
        142
    );
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 54708)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
                .into()
        ),
        281
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 54087)
}

fn get_input(custom_input: Option<&str>) -> String {
    String::from(custom_input.unwrap_or(PUZZLE_INPUT))
}
