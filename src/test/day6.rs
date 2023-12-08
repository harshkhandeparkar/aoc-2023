use crate::solutions::day6::solution;

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "Time:      7  15   30
Distance:  9  40  200"
                .into()
        ),
        288
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 4811940)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "Time:      7  15   30
Distance:  9  40  200"
                .into()
        ),
        71503
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 30077773)
}
