use aoc_2024::*;
use regex::Regex;

fn main() {
    let file_str = read_file_lines(get_input_file())
        .into_iter()
        .reduce(|acc, new| acc + &new)
        .unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut result: i32 = 0;

    for (_, [op1, op2]) in re.captures_iter(&file_str).map(|c| c.extract()) {
        result += op1.parse::<i32>().unwrap() * op2.parse::<i32>().unwrap();
    }

    println!("Result: {result}");
}
