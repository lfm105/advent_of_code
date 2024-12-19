use aoc_2024::*;
use regex::Regex;

enum Operation {
    Enable,
    Disable,
    Mul(i32, i32),
}

fn main() {
    let file_str = read_file_lines(get_input_file())
        .into_iter()
        .reduce(|acc, new| acc + &new)
        .unwrap();

    let mut operations: Vec<(usize, Operation)> = Vec::new();

    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    do_re.find_iter(&file_str).into_iter().for_each(|m| {
        operations.push((m.range().start, Operation::Enable));
    });

    dont_re.find_iter(&file_str).into_iter().for_each(|m| {
        operations.push((m.range().start, Operation::Disable));
    });

    mul_re
        .captures_iter(&file_str)
        .into_iter()
        .for_each(|capture| {
            let (_, [op1, op2]) = capture.extract();
            operations.push((
                capture.get(0).unwrap().start(),
                Operation::Mul(op1.parse::<i32>().unwrap(), op2.parse::<i32>().unwrap()),
            ));
        });

    operations.sort_by(|op1, op2| op1.0.cmp(&op2.0));

    let mut mul_enabled: bool = true;
    let mut result: i32 = 0;

    operations.into_iter().for_each(|(idx, op)| match op {
        Operation::Enable => mul_enabled = true,
        Operation::Disable => mul_enabled = false,
        Operation::Mul(a, b) => {
            if mul_enabled {
                result += a * b
            }
        }
    });

    println!("Result: {result}");
}
