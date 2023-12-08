use crate::solutions::day1::solution;

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
