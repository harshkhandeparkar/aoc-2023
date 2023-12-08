use crate::solutions::day7::solution;

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
