mod day1;

pub fn get_solution(day: i32) {
	match day {
		1 => day1::solution(),
		_ => println!("Invalid day.")
	}
}