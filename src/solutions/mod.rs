mod day1;
mod day2;
mod day3;
mod day4;

pub fn get_solution(day: u32, part: u32) {
    match day {
        1 => day1::solution(part),
        2 => day2::solution(part),
        3 => day3::solution(part),
        4 => day4::solution(part),
        _ => println!("Invalid day."),
    }
}
