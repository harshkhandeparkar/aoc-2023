use crate::inputs::day11::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> i128 {
	let expansion_rate: i128 = if part == 1 {
		2
	} else {
		1000_000
	};

    let (_, galaxy_locations, empty_row_indexes, empty_col_indexes) = get_input(custom_input);

	galaxy_locations.iter().enumerate().map(|(i, loc1)| {
		galaxy_locations[(i+1)..].iter().map(|loc2| {
			let i_min = std::cmp::min(loc1.0, loc2.0);
			let i_max = std::cmp::max(loc1.0, loc2.0);

			let j_min = std::cmp::min(loc1.1, loc2.1);
			let j_max = std::cmp::max(loc1.1, loc2.1);

			let num_expanded_rows = empty_row_indexes.iter().filter(|&&i| i_min < i && i < i_max).count() as i128;
			let num_expanded_cols = empty_col_indexes.iter().filter(|&&j| j_min < j && j < j_max).count() as i128;

			(i_max - i_min) as i128 + (j_max - j_min) as i128 + (num_expanded_rows + num_expanded_cols) * (expansion_rate - 1)
		}).sum::<i128>()
	}).sum::<i128>()
}

fn get_input(custom_input: Option<&str>) -> (Vec<Vec<char>>, Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);
    let cosmos: Vec<Vec<char>> = raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let expanded_row_indexes: Vec<usize> = cosmos
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|&tile| tile == '.'))
        .map(|(i, _)| i)
        .collect();

    let mut expanded_col_indexes: Vec<usize> = Vec::new();
    for i in 0..cosmos[0].len() {
        if cosmos.iter().all(|line| line[i] == '.') {
            expanded_col_indexes.push(i);
        }
    }


    let galaxy_locations: Vec<(usize, usize)> = cosmos
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == '#')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    (cosmos, galaxy_locations, expanded_row_indexes, expanded_col_indexes)
}
