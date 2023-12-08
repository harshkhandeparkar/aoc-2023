use std::collections::HashSet;

use crate::inputs::day3::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u32 {
    let mut grid = get_input(custom_input);

    let mut part_values: Vec<u32> = Vec::new();
    let mut current_part_value: u32 = 0;
    let mut is_num_adjacent: bool = false;

    for row in grid.iter_mut() {
        if is_num_adjacent {
            part_values.push(current_part_value);
        }

        current_part_value = 0;
        is_num_adjacent = false;

        for point in row.iter_mut() {
            match point {
                Point::None
                | Point::Star {
                    adjacent_part_ids: _,
                }
                | Point::Symbol => {
                    if is_num_adjacent {
                        part_values.push(current_part_value);
                    }

                    current_part_value = 0;
                    is_num_adjacent = false;
                }
                Point::Digit {
                    part_id,
                    value,
                    adjacent_to_symbol,
                } => {
                    is_num_adjacent = is_num_adjacent || *adjacent_to_symbol;
                    current_part_value = current_part_value * 10 + *value;

                    *part_id = part_values.len();
                }
            };
        }
    }

    if part == 1 {
        part_values.iter().sum::<u32>()
    } else {
        let num_rows = grid.len();
        let num_cols = grid[0].len();

        for row in 0..num_rows {
            for col in 0..num_cols {
                let point = grid[row][col].clone();

                if let Point::Star { adjacent_part_ids } = point {
                    let mut new_adj_part_ids = adjacent_part_ids.clone();

                    for i in -1..=1 {
                        for j in -1..=1 {
                            if i == 0 && j == 0 {
                                continue;
                            }

                            if (row == 0 && i == -1) || (row == num_rows - 1 && i == 1) {
                                continue;
                            }

                            if (col == 0 && j == -1) || (col == num_cols - 1 && j == 1) {
                                continue;
                            }

                            if let Point::Digit {
                                value: _,
                                adjacent_to_symbol: _,
                                part_id,
                            } = grid[(row as i32 + i) as usize][((col as i32) + j) as usize]
                            {
                                new_adj_part_ids.insert(part_id);
                            }
                        }
                    }
                    grid[row][col] = Point::Star {
                        adjacent_part_ids: new_adj_part_ids,
                    }
                }
            }
        }

        let mut total_gear_ratio = 0;

        for row in grid.iter_mut() {
            for point in row.iter_mut() {
                if let Point::Star { adjacent_part_ids } = point {
                    if adjacent_part_ids.len() == 2 {
                        let adjacent_part_ids: Vec<usize> =
                            adjacent_part_ids.iter().copied().collect();

                        total_gear_ratio += part_values[*adjacent_part_ids.first().unwrap()]
                            * part_values[*adjacent_part_ids.last().unwrap()];
                    }
                }
            }
        }

        total_gear_ratio
    }
}

#[derive(Clone, Debug)]
enum Point {
    Digit {
        value: u32,
        adjacent_to_symbol: bool,
        part_id: usize,
    },
    Star {
        adjacent_part_ids: HashSet<usize>,
    },
    Symbol,
    None,
}
fn get_input(custom_input: Option<&str>) -> Vec<Vec<Point>> {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);

    let mut grid: Vec<Vec<Point>> = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| {
                    if let Some(value) = character.to_digit(10) {
                        Point::Digit {
                            part_id: 0,
                            value,
                            adjacent_to_symbol: false,
                        }
                    } else if character == '.' {
                        Point::None
                    } else if character == '*' {
                        Point::Star {
                            adjacent_part_ids: HashSet::new(),
                        }
                    } else {
                        Point::Symbol
                    }
                })
                .collect()
        })
        .collect();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for row in 0..num_rows {
        for col in 0..num_cols {
            let point = grid[row][col].clone();

            if let Point::Digit {
                part_id: 0,
                value,
                adjacent_to_symbol: _,
            } = point
            {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        if (row == 0 && i == -1) || (row == num_rows - 1 && i == 1) {
                            continue;
                        }

                        if (col == 0 && j == -1) || (col == num_cols - 1 && j == 1) {
                            continue;
                        }

                        if let Point::Symbol =
                            grid[(row as i32 + i) as usize][((col as i32) + j) as usize]
                        {
                            grid[row][col] = Point::Digit {
                                part_id: 0,
                                value,
                                adjacent_to_symbol: true,
                            }
                        }

                        if let Point::Star {
                            adjacent_part_ids: _,
                        } = grid[(row as i32 + i) as usize][((col as i32) + j) as usize]
                        {
                            grid[row][col] = Point::Digit {
                                part_id: 0,
                                value,
                                adjacent_to_symbol: true,
                            }
                        }
                    }
                }
            }
        }
    }

    grid
}
