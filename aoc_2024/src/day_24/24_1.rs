use aoc_2024::*;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Copy, Clone)]
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

    let mut gates: HashMap<String, Gate> = HashMap::new();
    let mut values: BTreeMap<String, u8> = BTreeMap::new();

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

            gates.insert(
                captures[4].to_string(),
                (captures[1].to_string(), captures[3].to_string(), op),
            );
        }
    });

    gates.iter().for_each(|(string, gate)| {
        let res = do_operations(&gate, &gates, &mut values);
        values.insert(string.clone(), res);
    });

    let result = values.into_iter().filter(|(name, _)| name.starts_with("z")).rev().fold(0u64, |acc, (_, value)| {
        let mut result = acc << 1;
        result |= value as u64;
        result
    });

    println!("Resulting number: {result}");
}
