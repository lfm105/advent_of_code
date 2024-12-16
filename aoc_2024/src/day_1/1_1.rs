use aoc_2024::*;

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let mut column_1: Vec<i32> = Vec::with_capacity(input_lines.len());
    let mut column_2: Vec<i32> = Vec::with_capacity(input_lines.len());

    input_lines.into_iter().for_each(|l| {
        let mut split_line_iter = l.split_ascii_whitespace();
        column_1.push(split_line_iter.next().unwrap().parse().unwrap());
        column_2.push(split_line_iter.next().unwrap().parse().unwrap());
    });

    column_1.sort();
    column_2.sort();

    let total_difference: i32 = column_1
        .into_iter()
        .zip(column_2.into_iter())
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    println!("Total difference: {total_difference}");
}
