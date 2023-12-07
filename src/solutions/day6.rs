pub fn solution(part: u32) {
    let (time_values, distance_values) = get_input();

	if part == 1 {
		println!(
			"{}",
			time_values
				.iter()
				.enumerate()
				.map(|(i, time_allowed)| {
					let last_best_dist = distance_values[i];

					// distance travelled = x * (T - x) = xT - x^2 [x = time for which the button was held, T = time allowed]
					// if last best distance = c, last best time = t,
					// c = Tt - t^2 => t^2 - tT + c = 0
					// apply quadratic formula

					let d = (*time_allowed * (*time_allowed) - 4.0 * last_best_dist).sqrt();

					let button_hold_time_1 = (time_allowed - d) / 2.0;
					let button_hold_time_2 = (time_allowed + d) / 2.0;

					let beat_time_low = (button_hold_time_1 + 1.0).ceil()
						- (button_hold_time_1.ceil() - button_hold_time_1).ceil();
					let beat_time_high = (button_hold_time_2 - 1.0).floor()
						+ (button_hold_time_2.ceil() - button_hold_time_2).ceil();

					beat_time_high - beat_time_low + 1.0
				})
				.product::<f32>()
		)
	}
}

fn get_input() -> (Vec<f32>, Vec<f32>) {
    let raw_input = "Time:      7  15   30
	Distance:  9  40  200";

    let parsed_input = raw_input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num_str| num_str.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();

    (parsed_input[0].clone(), parsed_input[1].clone())
}
