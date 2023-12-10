use crate::solutions::day10::solution;

#[test]
fn check_part1_example1() {
    assert_eq!(
        solution(
            1,
            ".....
.S-7.
.|.|.
.L-J.
....."
                .into()
        ),
        4
    );
}

#[test]
fn check_part1_example2() {
    assert_eq!(
        solution(
            1,
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
                .into()
        ),
        8
    );
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 6846);
}

#[test]
fn check_part2() {
    // assert_eq!(solution(2, None), 0);
    panic!();
}
