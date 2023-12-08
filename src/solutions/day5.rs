use crate::inputs::day5::PUZZLE_INPUT;

pub fn solution(part: u32, custom_input: Option<&str>) -> u64 {
    let input = get_input(custom_input);

    let min_location = input
        .seeds
        .iter()
        .map(|seed| {
            let mut location = *seed;

            for map in input.maps.clone() {
                for range in map {
                    if range.src_start <= location && location <= range.src_end {
                        location = range.dst_start + location - range.src_start;
                        break;
                    }
                }
            }

            location
        })
        .min()
        .unwrap();

    if part == 1 {
        min_location
    } else {
        // Check if any locations less than min_location are feasible and return that
        let mut best_known_min_location = min_location;

        for test_location in 0..min_location {
            println!(
                "testing {}%",
                (test_location as f64 / min_location as f64) * 100.0
            );
            let mut traversed_location = test_location;
            let mut new_best_found = false;
            // Check feasibility `by applying the inverse transform
            for map in input.maps.clone().iter().rev() {
                for range in map {
                    if range.dst_start <= traversed_location && traversed_location <= range.dst_end
                    {
                        traversed_location = range.src_start + traversed_location - range.dst_start;
                        break;
                    }
                }
            }

            let mut seed_iter = input.seeds.iter();
            while let Some(seed_start) = seed_iter.next() {
                if let Some(seed_len) = seed_iter.next() {
                    if *seed_start <= traversed_location
                        && traversed_location < *seed_start + *seed_len
                    {
                        best_known_min_location = traversed_location;
                        new_best_found = true;
                        break;
                    }
                }
            }

            if new_best_found {
                break;
            }
        }

        best_known_min_location
    }
}

#[derive(Debug, Clone)]
struct ParsedInput {
    seeds: Vec<u64>,
    maps: Vec<Vec<Range>>,
}

#[derive(Debug, Clone, Copy)]
struct Range {
    src_start: u64,
    src_end: u64,
    dst_start: u64,
    dst_end: u64,
}

fn get_input(custom_input: Option<&str>) -> ParsedInput {
    let raw_input = custom_input.unwrap_or(PUZZLE_INPUT);

    let seeds = raw_input
        .lines()
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let maps = raw_input
        .split_once("\n\n")
        .unwrap()
        .1
        .split("\n\n")
        .map(|map| map.split_once(':').unwrap().1.trim())
        .map(|map| {
            map.lines()
                .map(|line| {
                    let mut range_iter = line
                        .split_whitespace()
                        .map(|num| num.parse::<u64>().unwrap());
                    let dst_start = range_iter.next().unwrap();
                    let src_start = range_iter.next().unwrap();
                    let length = range_iter.next().unwrap();

                    Range {
                        src_start,
                        src_end: src_start + length - 1,
                        dst_start,
                        dst_end: dst_start + length - 1,
                    }
                })
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<Vec<Range>>>();

    ParsedInput { seeds, maps }
}
