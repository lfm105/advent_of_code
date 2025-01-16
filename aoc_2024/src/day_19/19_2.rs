use aoc_2024::*;
use std::collections::{HashMap, HashSet};

fn ways_to_design<'a>(
    goal_design: &'a str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if goal_design.is_empty() {
        return 1;
    } else if cache.contains_key(goal_design) {
        return *cache.get(goal_design).unwrap();
    }

    let result = patterns
        .iter()
        .filter(|p| goal_design.starts_with(*p))
        .fold(0u64, |acc, p| {
            acc + ways_to_design(&goal_design[p.len()..], patterns, cache)
        });

    cache.insert(goal_design, result);

    result
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let mut input_lines_iter = input_lines.into_iter();

    let first_line = input_lines_iter.next().unwrap();

    let patterns: HashSet<&str> = first_line.split(", ").collect();

    input_lines_iter.next();

    let designs_owned: Vec<String> = input_lines_iter.collect();
    let designs_refs: Vec<&str> = designs_owned.iter().map(|line| line.as_str()).collect();

    let mut cache: HashMap<&str, u64> = HashMap::new();

    let possible_designs: u64 = designs_refs.into_iter().fold(0u64, |acc, design| {
        acc + ways_to_design(&design, &patterns, &mut cache)
    });

    println!("There are {possible_designs} ways to make all designs");
}
