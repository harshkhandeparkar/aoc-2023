mod day1;
mod day2;

pub fn get_solution(day: u32, part: u32) {
    match day {
        1 => day1::solution(part),
        2 => day2::solution(part),
        _ => println!("Invalid day."),
    }
}
