use std::cmp;
use std::collections::HashMap;

use crate::inputs::day2::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u32 {
    let input = get_input(custom_input);

    if part == 1 {
        let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        input
            .iter()
            .enumerate()
            .filter(|(_, game)| {
                let impossible_reveals: Vec<&HashMap<String, u32>> = game
                    .iter()
                    .filter(|reveal| {
                        reveal.get("red").unwrap() > max_cubes.get("red").unwrap()
                            || reveal.get("green").unwrap() > max_cubes.get("green").unwrap()
                            || reveal.get("blue").unwrap() > max_cubes.get("blue").unwrap()
                    })
                    .collect();

                impossible_reveals.is_empty()
            })
            .map(|(game_i, _)| game_i as u32 + 1)
            .sum::<u32>()
    } else {
        input
            .iter()
            .map(|game| {
                let mut min_cubes: HashMap<String, u32> =
                    HashMap::from([("red".into(), 0), ("green".into(), 0), ("blue".into(), 0)]);

                for reveal in game {
                    for color in ["red".to_string(), "green".to_string(), "blue".to_string()] {
                        min_cubes.insert(
                            color.clone(),
                            cmp::max(
                                *min_cubes.get(&color).unwrap(),
                                *reveal.get(&color).unwrap(),
                            ),
                        );
                    }
                }

                min_cubes.get("red").unwrap()
                    * min_cubes.get("green").unwrap()
                    * min_cubes.get("blue").unwrap()
            })
            .sum::<u32>()
    }
}

#[test]
fn check_example_part1() {
    assert_eq!(
        solution(
            1,
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .into()
        ),
        8
    )
}

#[test]
fn check_part1() {
    assert_eq!(solution(1, None), 2720)
}

#[test]
fn check_example_part2() {
    assert_eq!(
        solution(
            2,
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .into()
        ),
        2286
    )
}

#[test]
fn check_part2() {
    assert_eq!(solution(2, None), 71535)
}

fn get_input(custom_input: Option<&str>) -> Vec<Vec<HashMap<String, u32>>> {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);

    raw_input
        .lines()
        .map(|line| {
            let reveals = line.split(':').last().unwrap().trim();

            reveals
                .split(';')
                .map(|reveal| {
                    let cubes: Vec<(String, u32)> = reveal
                        .trim()
                        .split(',')
                        .map(|cube| {
                            cube.trim()
                                .split(' ')
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()
                        })
                        .map(|cube_info| {
                            (
                                cube_info.last().unwrap().clone().trim().to_string(),
                                cube_info
                                    .first()
                                    .unwrap()
                                    .clone()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                            )
                        })
                        .collect();

                    let mut cube_map: HashMap<String, u32> =
                        HashMap::from([("red".into(), 0), ("green".into(), 0), ("blue".into(), 0)]);

                    for cube in cubes {
                        cube_map.insert(cube.0.clone(), cube_map.get(&cube.0).unwrap() + cube.1);
                    }

                    cube_map
                })
                .collect()
        })
        .collect()
}
