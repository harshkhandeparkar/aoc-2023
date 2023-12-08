mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub fn get_solution(day: u32, part: u32) {
    println!(
        "{}",
        match day {
            1 => day1::solution(part, None) as u128,
            2 => day2::solution(part, None) as u128,
            3 => day3::solution(part, None) as u128,
            4 => day4::solution(part, None),
            5 => day5::solution(part, None) as u128,
            6 => day6::solution(part, None) as u128,
            7 => day7::solution(part, None),
            8 => day8::solution(part, None),
            _ => 0,
        }
    );
}
