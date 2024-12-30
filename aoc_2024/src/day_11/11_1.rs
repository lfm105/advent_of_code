use aoc_2024::*;

fn split_digits(stone: u64) -> (u64, u64) {
    let number_of_digits = number_of_digits(stone);

    (stone / 10u64.pow(number_of_digits / 2), stone % 10u64.pow(number_of_digits / 2))
}

fn number_of_digits(stone: u64) -> u32 {
    stone.ilog10() + 1
}

fn calculate_number_of_stones(curr_stone: u64, curr_depth: u32, target_depth: u32) -> u64 {
    if curr_depth == target_depth {
        1
    } else {
        match curr_stone {
            0 => calculate_number_of_stones(1, curr_depth + 1, target_depth),
            _ => match number_of_digits(curr_stone) % 2 {
                0 => {
                    let digits = split_digits(curr_stone);
                    calculate_number_of_stones(digits.0, curr_depth + 1, target_depth)
                        + calculate_number_of_stones(digits.1, curr_depth + 1, target_depth)
                }
                1 => calculate_number_of_stones(curr_stone * 2024, curr_depth + 1, target_depth),
                _ => panic!(),
            },
        }
    }
}

fn main() {
    let stones = read_file_lines(get_input_file())
        .into_iter()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let result = stones.into_iter().fold(0u64, |acc, stone| {
        acc + calculate_number_of_stones(stone, 0, 25)
    });

    println!("Result: {result}");
}
