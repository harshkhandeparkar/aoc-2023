use crate::solutions::day10::solution;

#[test]
fn check_part1_example() {
    assert_eq!(
        solution(
            1,
            ".....
.F-7.
.|.|.
.L-J.
....."
                .into()
        ),
        0
    );
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 0)
}

#[test]
fn check_part2_example() {
    assert_eq!(
        solution(
            2,
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
                .into()
        ),
        2
    );
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 0);
}
