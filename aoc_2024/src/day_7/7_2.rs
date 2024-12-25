use aoc_2024::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

fn do_operation(first_operand: i64, second_operand: i64, operation: &Operation) -> i64 {
    match operation {
        Operation::Addition => first_operand + second_operand,
        Operation::Multiplication => first_operand * second_operand,
        Operation::Concatenation => {
            first_operand * 10_i64.pow(second_operand.ilog10() + 1) + second_operand
        }
    }
}

fn try_operations(
    target_value: &i64,
    curr_value: Option<i64>,
    values: &[i64],
    curr_operation: Option<Operation>,
) -> Option<()> {
    if values.is_empty() {
        if let Some(c_value) = curr_value {
            if c_value == *target_value {
                return Some(());
            }
        } else {
            return None;
        }
    } else if let (Some(c_value), Some(c_op)) = (curr_value, curr_operation) {
        let res = Operation::iter().any(|op| {
            try_operations(
                &target_value,
                Some(do_operation(c_value, values[0], &c_op)),
                &values[1..],
                Some(op),
            ) != None
        });

        if res {
            return Some(());
        }
    } else {
        let res = Operation::iter().any(|op| {
            try_operations(&target_value, Some(values[0]), &values[1..], Some(op)) != None
        });

        if res {
            return Some(());
        }
    }

    None
}

fn main() {
    let lines = read_file_lines(get_input_file());
    let map: Vec<(i64, Vec<i64>)> = lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(':');
            let result = split.next().unwrap().parse::<i64>().unwrap();
            let values = split
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            (result, values)
        })
        .collect();

    let sum = map.into_iter().fold(0, |acc, (result, values)| {
        if let Some(_) = try_operations(&result, None, &values, None) {
            return acc + result;
        }

        acc
    });

    println!("Sum of valid calibration results: {sum}");
}
