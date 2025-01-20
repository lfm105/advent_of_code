use aoc_2024::*;

fn process_secret_number(mut secret_number: u64, times: usize) -> u64 {
    if times == 0 {
        return secret_number;
    } else {
        secret_number ^= secret_number << 6;
        secret_number &= (1 << 24) - 1;
        secret_number ^= secret_number >> 5;
        secret_number &= (1 << 24) - 1;
        secret_number ^= secret_number << 11;
        secret_number &= (1 << 24) - 1;
        return process_secret_number(secret_number, times - 1);
    }
}

fn main() {
    process_secret_number(123, 10);

    let secret_numbers = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let processed_secret_numbers = secret_numbers
        .into_iter()
        .map(|secret_number| process_secret_number(secret_number, 2000))
        .collect::<Vec<_>>();

    let sum = processed_secret_numbers
        .into_iter()
        .fold(0u64, |acc, processed_secret_number| {
            acc + processed_secret_number
        });

    println!("Sum of secret numbers after being processed 2000 times: {sum}");
}
