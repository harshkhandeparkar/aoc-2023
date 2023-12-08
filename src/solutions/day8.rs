use crate::inputs::day8::PUZZLE_INPUT;
use std::collections::HashMap;

pub fn solution(part: u32, custom_input: Option<&str>) -> u128 {
    let input = get_input(custom_input);

    let mut positions: Vec<String> = if part == 1 {
        vec!["AAA".into()]
    } else {
        input
            .network
            .keys()
            .filter(|pos| pos.ends_with('A'))
            .cloned()
            .collect()
    };

    let mut num_steps_arr: Vec<u128> = Vec::new();

    for position in &mut positions {
        let mut num_steps = 0;
        let mut direction_iter = input.directions.iter().cycle();

        while !position.ends_with('Z') {
            let dir = *direction_iter.next().unwrap();
            num_steps += 1;

            let posn_net = input.network.get(position).unwrap();

            *position = if dir == 'L' {
                posn_net.0.clone()
            } else {
                posn_net.1.clone()
            }
        }

        num_steps_arr.push(num_steps);
    }

    lcm(&num_steps_arr)
}

#[derive(Debug)]
struct ParsedInput {
    directions: Vec<char>,
    network: HashMap<String, (String, String)>,
}
fn get_input(custom_input: Option<&str>) -> ParsedInput {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);

    let (directions_str, network_str) = raw_input.split_once("\n\n").unwrap();

    let mut network = HashMap::new();

    for line in network_str.trim().lines() {
        let mut line_split = line.split_whitespace();

        let network_from = line_split
            .next()
            .unwrap()
            .trim()
            .chars()
            .filter(|character| character.is_alphanumeric())
            .collect();
        let network_to_l = line_split
            .nth(1)
            .unwrap()
            .trim()
            .chars()
            .filter(|character| character.is_alphanumeric())
            .collect();
        let network_to_r = line_split
            .next()
            .unwrap()
            .trim()
            .chars()
            .filter(|character| character.is_alphanumeric())
            .collect();

        network.insert(network_from, (network_to_l, network_to_r));
    }

    ParsedInput {
        directions: directions_str.trim().chars().collect(),
        network,
    }
}

// copied from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
