use crate::solutions::day3::solution;

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .into()
        ),
        4361
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 540212)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
                .into()
        ),
        467835
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 87605697)
}
