use crate::inputs::day9::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> i128 {
    let input = get_input(custom_input);

    input
        .iter()
        .map(|history| {
            let mut differences: Vec<Vec<i128>> = Vec::new();
            let mut prev_diff_seq = history.clone();
            differences.push(prev_diff_seq.clone());

            let mut new_diff_seq = Vec::new();

            for diff_seq_len in (1..history.len()).rev() {
                for i in 1..=diff_seq_len {
                    new_diff_seq.push(prev_diff_seq[i] - prev_diff_seq[i - 1]);
                }

                differences.push(new_diff_seq.clone());

                if new_diff_seq.iter().all(|&val| val == 0) {
                    break;
                }

                prev_diff_seq = new_diff_seq;
                new_diff_seq = Vec::new();
            }

            if part == 1 {
                differences
                    .iter()
                    .fold(0, |acc, elem| acc + elem.last().unwrap())
            } else {
                let mut extrapolated_value = 0;

                for i in (0..(differences.len() - 1)).rev() {
                    extrapolated_value = differences[i].first().unwrap() - extrapolated_value;
                }

                extrapolated_value
            }
        })
        .sum::<i128>()
}

fn get_input(custom_input: Option<&str>) -> Vec<Vec<i128>> {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);

    raw_input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i128>().unwrap())
                .collect()
        })
        .collect()
}
