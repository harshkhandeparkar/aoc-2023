use crate::solutions::day8::solution;

#[test]
fn check_part1_example1() {
    assert_eq!(
        solution(
            1,
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
                .into()
        ),
        2
    )
}

#[test]
fn check_part1_example2() {
    assert_eq!(
        solution(
            1,
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                .into()
        ),
        6
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 12599)
}

#[test]
fn check_part2_example1() {
    assert_eq!(
        solution(
            2,
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
                .into()
        ),
        6
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 8245452805243)
}
