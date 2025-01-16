use aoc_2024::*;
use std::collections::{HashMap, HashSet};

fn design_possible<'a>(
    goal_design: &'a str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if goal_design.is_empty() {
        return true;
    } else if cache.contains_key(goal_design) {
        return *cache.get(goal_design).unwrap();
    }

    let result = patterns
        .iter()
        .filter(|p| goal_design.starts_with(*p))
        .find(|p| design_possible(&goal_design[p.len()..], patterns, cache))
        != None;

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

    let mut cache: HashMap<&str, bool> = HashMap::new();

    let possible_designs: Vec<&str> = designs_refs
        .into_iter()
        .filter(|design| design_possible(&design, &patterns, &mut cache))
        .collect();

    println!("There are {} possible designs", possible_designs.len());
}
