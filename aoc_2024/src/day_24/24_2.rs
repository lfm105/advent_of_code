use aoc_2024::*;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use strum_macros::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
enum Operation {
    AND,
    OR,
    XOR,
}

type Gate = (String, String, Operation);

fn do_operations(
    current_gate: &Gate,
    gates: &HashMap<String, Gate>,
    values: &mut BTreeMap<String, u8>,
) -> u8 {
    if !values.contains_key(&current_gate.0) {
        let first_operation_result =
            do_operations(gates.get(&current_gate.0).unwrap(), gates, values);
        values.insert(current_gate.0.clone(), first_operation_result);
    }
    if !values.contains_key(&current_gate.1) {
        let second_operation_result =
            do_operations(gates.get(&current_gate.1).unwrap(), gates, values);
        values.insert(current_gate.1.clone(), second_operation_result);
    }

    match &current_gate.2 {
        Operation::AND => {
            return values.get(&current_gate.0).unwrap() & values.get(&current_gate.1).unwrap()
        }
        Operation::OR => {
            return values.get(&current_gate.0).unwrap() | values.get(&current_gate.1).unwrap()
        }
        Operation::XOR => {
            return values.get(&current_gate.0).unwrap() ^ values.get(&current_gate.1).unwrap()
        }
    }
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let initial_value_regex = Regex::new(r"([a-z0-9]+): ([0|1])").unwrap();
    let operation_regex =
        Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();

    let mut values: BTreeMap<String, u8> = BTreeMap::new();
    let mut gates: HashMap<String, Gate> = HashMap::new();
    let mut faulty_outputs: BTreeSet<String> = BTreeSet::new();

    let mut i = 0u32;

    input_lines.into_iter().for_each(|line| {
        if let Some(captures) = initial_value_regex.captures(&line) {
            values.insert(captures[1].to_string(), captures[2].parse::<u8>().unwrap());
        } else if let Some(captures) = operation_regex.captures(&line) {
            let op = match &captures[2] {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "XOR" => Operation::XOR,
                _ => panic!(),
            };

            println!("{} -> {}{}", captures[1].to_string(), op, i);
            println!("{} -> {}{}", captures[3].to_string(), op, i);
            println!("{}{} -> {}", op, i, captures[4].to_string());

            if captures[4].starts_with("z") && captures[4] != *"z45" && op != Operation::XOR {
                faulty_outputs.insert(captures[4].to_string());
            }

            if !captures[4].starts_with("z")
                && op == Operation::XOR
                && !((captures[1].starts_with("x") && captures[3].starts_with("y"))
                    || (captures[1].starts_with("y") && captures[3].starts_with("x")))
            {
                faulty_outputs.insert(captures[4].to_string());
            }

            i += 1;

            gates.insert(
                captures[4].to_string(),
                (captures[1].to_string(), captures[3].to_string(), op),
            );
        }
    });

    dbg!(&faulty_outputs);

    gates.iter().for_each(|(string, gate)| {
        let res = do_operations(&gate, &gates, &mut values);
        values.insert(string.clone(), res);
    });

    let x = values
        .iter()
        .filter(|(name, _)| name.starts_with("x"))
        .rev()
        .fold(0u64, |mut acc, (_, value)| {
            acc <<= 1;
            acc |= *value as u64;
            acc
        });

    let y = values
        .iter()
        .filter(|(name, _)| name.starts_with("y"))
        .rev()
        .fold(0u64, |mut acc, (_, value)| {
            acc <<= 1;
            acc |= *value as u64;
            acc
        });

    let expected_z = x + y;

    let obtained_z = values
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .rev()
        .fold(0u64, |mut acc, (_, value)| {
            acc <<= 1;
            acc |= *value as u64;
            acc
        });

    println!("Expected z: {expected_z:b}");
    println!("Obtained z: {obtained_z:b}");
}
