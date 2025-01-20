use aoc_2024::*;
use std::collections::HashMap;

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
    let secret_numbers = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let prices: Vec<Vec<u8>> = secret_numbers
        .into_iter()
        .map(|mut secret_number| {
            let mut monkey_prices = vec![];

            for _ in 0..2000 {
                monkey_prices.push((secret_number % 10) as u8);
                secret_number = process_secret_number(secret_number, 1);
            }

            monkey_prices
        })
        .collect();

    let prices_and_deltas: Vec<Vec<(u8, u8)>> = prices
        .into_iter()
        .map(|price_vec| {
            let mut pd = vec![];

            let mut previous_price = 0u8;

            for price in price_vec {
                pd.push((price, price + 9 - previous_price));
                previous_price = price;
            }

            pd
        })
        .collect();

    let mut total_bananas_per_sequence: HashMap<u32, u64> = HashMap::new();

    prices_and_deltas.into_iter().for_each(|pd| {
        let mut bananas_per_sequence: HashMap<u32, u8> = HashMap::new();

        let mut sequence_idx = 0u32;

        for i in 0..pd.len() {
            sequence_idx |= pd[i].1 as u32;

            let bananas = pd[i].0;

            if i >= 4 {
                if !bananas_per_sequence.contains_key(&sequence_idx) {
                    bananas_per_sequence.insert(sequence_idx, bananas);
                }
            }

            sequence_idx <<= 5;
            sequence_idx &= 0b11111111111111111111;
        }

        bananas_per_sequence
            .into_iter()
            .for_each(|(sequence_idx, bananas)| {
                if !total_bananas_per_sequence.contains_key(&sequence_idx) {
                    total_bananas_per_sequence.insert(sequence_idx, 0);
                }

                *total_bananas_per_sequence.get_mut(&sequence_idx).unwrap() += bananas as u64;
            });
    });

    let mut most_bananas = 0u64;

    for (_, v) in &total_bananas_per_sequence {
        if *v > most_bananas {
            most_bananas = *v;
        }
    }

    println!("The most bananas you can get is {most_bananas}");
}
