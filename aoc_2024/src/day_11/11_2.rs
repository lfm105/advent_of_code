use aoc_2024::*;
use std::collections::HashMap;

fn split_digits(stone: u64) -> (u64, u64) {
    let number_of_digits = number_of_digits(stone);

    (stone / 10u64.pow(number_of_digits / 2), stone % 10u64.pow(number_of_digits / 2))
}

fn number_of_digits(stone: u64) -> u32 {
    stone.ilog10() + 1
}

fn calculate_number_of_stones(curr_stone: u64, curr_depth: u32, target_depth: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    let retval: u64;

    if curr_depth == target_depth {
        retval = 1;
    } else {
        match cache.get(&(curr_stone, curr_depth)) {
            None => match curr_stone {
                0 => retval = calculate_number_of_stones(1, curr_depth + 1, target_depth, cache),
                _ => match number_of_digits(curr_stone) % 2 {
                    0 => {
                        let digits = split_digits(curr_stone);
                        retval = calculate_number_of_stones(digits.0, curr_depth + 1, target_depth, cache)
                            + calculate_number_of_stones(digits.1, curr_depth + 1, target_depth, cache);
                    }
                    1 => retval = calculate_number_of_stones(curr_stone * 2024, curr_depth + 1, target_depth, cache),
                    _ => panic!(),
                },
            },
            Some(stones) => {
                retval = *stones;
            }
        }     
    }

    cache.insert((curr_stone, curr_depth), retval);

    retval
}

fn main() {
    let stones = read_file_lines(get_input_file())
        .into_iter()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();

    let result = stones.into_iter().fold(0u64, |acc, stone| {
        acc + calculate_number_of_stones(stone, 0, 75, &mut cache)
    });

    println!("Result: {result}");
}
