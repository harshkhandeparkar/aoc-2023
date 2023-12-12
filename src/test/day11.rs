use crate::solutions::day11::solution;

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
fn check_part2_example1() {
    assert_eq!(
        solution(
            2,
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
                .into()
        ),
        4
    )
}

#[test]
fn check_part2_example2() {
    assert_eq!(
        solution(
            2,
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
                .into()
        ),
        8
    )
}

#[test]
fn check_part2_example3() {
    assert_eq!(
        solution(
            2,
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
                .into()
        ),
        10
    )
}

#[test]
fn check_part2() {
    // assert_eq!(solution(2, None), 0);
    panic!();
}
