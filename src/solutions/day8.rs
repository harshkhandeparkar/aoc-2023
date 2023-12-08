use crate::inputs::day8::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u128 {
    let input = get_input(custom_input);

    0
}

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

fn get_input(custom_input: Option<&str>) {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);
}
