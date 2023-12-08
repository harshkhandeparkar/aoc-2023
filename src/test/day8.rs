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
