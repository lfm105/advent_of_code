use aoc_2024::*;
use std::collections::HashSet;

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let mut column_1_set: HashSet<u32> = HashSet::new();
    let mut column_2: Vec<u32> = Vec::with_capacity(input_lines.len());

    input_lines.into_iter().for_each(|l| {
        let mut split_line_iter = l.split_ascii_whitespace();
        column_1_set.insert(split_line_iter.next().unwrap().parse().unwrap());
        column_2.push(split_line_iter.next().unwrap().parse().unwrap());
    });

    let similarity_score: u32 = column_2.into_iter().fold(0, |acc, x| {
        if column_1_set.contains(&x) {
            acc + &x
        } else {
            acc
        }
    });

    println!("Similarity score: {similarity_score}");
}
