use aoc_2024::*;
use std::collections::HashMap;

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let mut column_1: Vec<u32> = Vec::with_capacity(input_lines.len());
    let mut column_2: Vec<u32> = Vec::with_capacity(input_lines.len());

    input_lines.into_iter().for_each(|l| {
        let mut split_line_iter = l.split_ascii_whitespace();
        column_1.push(split_line_iter.next().unwrap().parse().unwrap());
        column_2.push(split_line_iter.next().unwrap().parse().unwrap());
    });

    let mut column_1_in_2_count: HashMap<u32, u32> = HashMap::new();

    column_1.iter().for_each(|n_1| {
        if let None = column_1_in_2_count.get(&n_1) {
            column_2.iter().for_each(|n_2| {
                if n_1 == n_2 {
                    if let Some(n_1_count) = column_1_in_2_count.get(n_1) {
                        column_1_in_2_count.insert(*n_1, n_1_count + 1);
                    } else {
                        column_1_in_2_count.insert(*n_1, 1);
                    }
                }
            })
        }
    });

    let similarity_score: u32 = column_1.into_iter().fold(0, |acc, x| {
        acc + x * column_1_in_2_count.get(&x).unwrap_or(&0)
    });

    println!("Similarity score: {similarity_score}");
}
