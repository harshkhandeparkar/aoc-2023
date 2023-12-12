use crate::solutions::day11::solution;

#[test]
fn check_part1_example() {
    assert_eq!(
        solution(
            1,
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                .into()
        ),
        374
    );
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 9599070);
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 842645913794);
}
