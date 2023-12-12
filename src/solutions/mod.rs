pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn get_solution(day: u32, part: u32) {
    println!(
        "{}",
        match day {
            1 => day1::solution(part, None) as i128,
            2 => day2::solution(part, None) as i128,
            3 => day3::solution(part, None) as i128,
            4 => day4::solution(part, None),
            5 => day5::solution(part, None) as i128,
            6 => day6::solution(part, None) as i128,
            7 => day7::solution(part, None),
            8 => day8::solution(part, None),
            9 => day9::solution(part, None),
            10 => day10::solution(part, None),
            11 => day11::solution(part, None),
            _ => 0,
        }
    );
}
