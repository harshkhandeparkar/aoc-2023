use std::io;
use std::io::prelude::*;

mod solutions;
mod inputs;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    print!("Enter day number: ");
    stdout.flush().unwrap();
    let mut day_line = String::new();
    stdin.lock().read_line(&mut day_line).unwrap();
    let day: u32 = day_line.trim().parse().unwrap();

    print!("Enter part number: ");
    stdout.flush().unwrap();
    let mut part_line = String::new();
    stdin.lock().read_line(&mut part_line).unwrap();
    let part: u32 = part_line.trim().parse().unwrap();

    solutions::get_solution(day, part);
}
