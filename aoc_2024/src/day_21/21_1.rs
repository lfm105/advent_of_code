use core::hash;
use std::collections::{HashMap, HashSet};

use aoc_2024::*;
use itertools::Itertools;

fn find_fastest_paths(
    keypad_fastest_paths: &HashMap<(char, char), HashSet<Vec<char>>>,
    controller_fastest_paths: &HashMap<(char, char), HashSet<Vec<char>>>,
    chars: &Vec<char>,
    current_depth: u32,
    target_depth: u32,
) -> HashSet<Vec<char>> {
    if current_depth == target_depth {
        return HashSet::from([chars.clone()]);
    }

    let mut fastest_paths = match current_depth {
        0 => keypad_fastest_paths.get(&('A', chars[0])).unwrap().clone(),
        _ => controller_fastest_paths
            .get(&('A', chars[0]))
            .unwrap()
            .clone(),
    };

    for i in 1..chars.len() {
        let mut new_fastest_paths: HashSet<Vec<char>> = HashSet::new();

        fastest_paths.into_iter().for_each(|fastest_path| {
            let current_char = chars[i - 1];
            let next_char = chars[i];
            let fastest_paths_from_current_to_next = match current_depth {
                0 => keypad_fastest_paths
                    .get(&(current_char, next_char))
                    .unwrap(),
                _ => controller_fastest_paths
                    .get(&(current_char, next_char))
                    .unwrap(),
            };
            fastest_paths_from_current_to_next
                .into_iter()
                .for_each(|fpfcn| {
                    new_fastest_paths.insert(
                        fastest_path
                            .iter()
                            .chain(fpfcn.iter())
                            .map(|c| *c)
                            .collect::<Vec<char>>(),
                    );
                });
        });

        fastest_paths = new_fastest_paths;
    }

    let mut next_set: HashSet<Vec<char>> = HashSet::new();

    fastest_paths.iter().for_each(|fastest_path| {
        let next_fastest_paths = find_fastest_paths(
            keypad_fastest_paths,
            controller_fastest_paths,
            fastest_path,
            current_depth + 1,
            target_depth,
        );
        next_fastest_paths
            .into_iter()
            .for_each(|next_fastest_path| {
                next_set.insert(next_fastest_path);
            });
    });

    next_set
}

fn main() {
    #[rustfmt::skip]
    let keypad_fastest_paths: HashMap<(char, char), HashSet<Vec<char>>> = HashMap::from([
        (('0', '0'), HashSet::from([vec!['A']])),
        (('0', '1'), HashSet::from([vec!['^', '<', 'A']])),
        (('0', '2'), HashSet::from([vec!['^', 'A']])),
        (('0', '3'), HashSet::from([vec!['^', '>', 'A'], vec!['>', '^', 'A']])),
        (('0', '4'), HashSet::from([vec!['^', '^', '<', 'A']])),
        (('0', '5'), HashSet::from([vec!['^', '^', 'A']])),
        (('0', '6'), HashSet::from([vec!['^', '^', '>', 'A'], vec!['>', '^', '^', 'A']])),
        (('0', '7'), HashSet::from([vec!['^', '^', '^', '<', 'A']])),
        (('0', '8'), HashSet::from([vec!['^', '^', '^', 'A']])),
        (('0', '9'), HashSet::from([vec!['^', '^', '^', '>', 'A'], vec!['>', '^', '^', '^', 'A']])),
        (('0', 'A'), HashSet::from([vec!['>', 'A']])),
        (('1', '0'), HashSet::from([vec!['>', 'v', 'A']])),
        (('1', '1'), HashSet::from([vec!['A']])),
        (('1', '2'), HashSet::from([vec!['>', 'A']])),
        (('1', '3'), HashSet::from([vec!['>', '>', 'A']])),
        (('1', '4'), HashSet::from([vec!['^', 'A']])),
        (('1', '5'), HashSet::from([vec!['^', '>', 'A'], vec!['>', '^', 'A']])),
        (('1', '6'), HashSet::from([vec!['^', '>', '>', 'A'], vec!['>', '>', '^', 'A']])),
        (('1', '7'), HashSet::from([vec!['^', '^', 'A']])),
        (('1', '8'), HashSet::from([vec!['^', '^', '>', 'A'], vec!['>', '^', '^', 'A']])),
        (('1', '9'), HashSet::from([vec!['^', '^', '>', '>', 'A'], vec!['>', '>', '^', '^', 'A']])),
        (('1', 'A'), HashSet::from([vec!['>', '>', 'v', 'A']])),
        (('2', '0'), HashSet::from([vec!['v', 'A']])),
        (('2', '1'), HashSet::from([vec!['<', 'A']])),
        (('2', '2'), HashSet::from([vec!['A']])),
        (('2', '3'), HashSet::from([vec!['>', 'A']])),
        (('2', '4'), HashSet::from([vec!['<', '^', 'A'], vec!['^', '<', 'A']])),
        (('2', '5'), HashSet::from([vec!['^', 'A']])),
        (('2', '6'), HashSet::from([vec!['^', '>'], vec!['>', '^', 'A']])),
        (('2', '7'), HashSet::from([vec!['^', '^', '<', 'A'], vec!['<', '^', '^', 'A']])),
        (('2', '8'), HashSet::from([vec!['^', '^', 'A']])),
        (('2', '9'), HashSet::from([vec!['^', '^', '>', 'A'], vec!['>', '^', '^', 'A']])),
        (('2', 'A'), HashSet::from([vec!['>', 'v', 'A'], vec!['v', '>', 'A']])),
        (('3', '0'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('3', '1'), HashSet::from([vec!['<', '<', 'A']])),
        (('3', '2'), HashSet::from([vec!['<', 'A']])),
        (('3', '3'), HashSet::from([vec!['A']])),
        (('3', '4'), HashSet::from([vec!['<', '<', '^', 'A'], vec!['^', '<', '<', 'A']])),
        (('3', '5'), HashSet::from([vec!['<', '^', 'A'], vec!['^', '<', 'A']])),
        (('3', '6'), HashSet::from([vec!['^', 'A']])),
        (('3', '7'), HashSet::from([vec!['^', '^', '<', '<', 'A'], vec!['<', '<', '^', '^', 'A']])),
        (('3', '8'), HashSet::from([vec!['^', '^', '<', 'A'], vec!['<', '^', '^', 'A']])),
        (('3', '9'), HashSet::from([vec!['^', '^', 'A']])),
        (('3', 'A'), HashSet::from([vec!['v', 'A']])),
        (('4', '0'), HashSet::from([vec!['>', 'v', 'v', 'A']])),
        (('4', '1'), HashSet::from([vec!['v', 'A']])),
        (('4', '2'), HashSet::from([vec!['v', '>', 'A'], vec!['>', 'v', 'A']])),
        (('4', '3'), HashSet::from([vec!['v', '>', '>', 'A'], vec!['>', '>', 'v', 'A']])),
        (('4', '4'), HashSet::from([vec!['A']])),
        (('4', '5'), HashSet::from([vec!['>', 'A']])),
        (('4', '6'), HashSet::from([vec!['>', '>', 'A']])),
        (('4', '7'), HashSet::from([vec!['^', 'A']])),
        (('4', '8'), HashSet::from([vec!['^', '>', 'A'], vec!['>', '^', 'A']])),
        (('4', '9'), HashSet::from([vec!['^', '>', '>', 'A'], vec!['>', '>', '^', 'A']])),
        (('4', 'A'), HashSet::from([vec!['>', '>', 'v', 'v', 'A']])),
        (('5', '0'), HashSet::from([vec!['v', 'v', 'A']])),
        (('5', '1'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('5', '2'), HashSet::from([vec!['v', 'A']])),
        (('5', '3'), HashSet::from([vec!['v', '>', 'A'], vec!['>', 'v', 'A']])),
        (('5', '4'), HashSet::from([vec!['<', 'A']])),
        (('5', '5'), HashSet::from([vec!['A']])),
        (('5', '6'), HashSet::from([vec!['>', 'A']])),
        (('5', '7'), HashSet::from([vec!['<', '^', 'A'], vec!['^', '<', 'A']])),
        (('5', '8'), HashSet::from([vec!['^', 'A']])),
        (('5', '9'), HashSet::from([vec!['^', '>', 'A'], vec!['>', '^', 'A']])),
        (('5', 'A'), HashSet::from([vec!['v', 'v', '>', 'A'], vec!['>', 'v', 'v', 'A']])),
        (('6', '0'), HashSet::from([vec!['<', 'v', 'v', 'A'], vec!['v', 'v', '<', 'A']])),
        (('6', '1'), HashSet::from([vec!['<', '<', 'v', 'A'], vec!['v', '<', '<', 'A']])),
        (('6', '2'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('6', '3'), HashSet::from([vec!['v', 'A']])),
        (('6', '4'), HashSet::from([vec!['<', '<', 'A']])),
        (('6', '5'), HashSet::from([vec!['<', 'A']])),
        (('6', '6'), HashSet::from([vec!['A']])),
        (('6', '7'), HashSet::from([vec!['<', '<', '^', 'A'], vec!['^', '<', '<', 'A']])),
        (('6', '8'), HashSet::from([vec!['<', '^', 'A'], vec!['^', '<', 'A']])),
        (('6', '9'), HashSet::from([vec!['^', 'A']])),
        (('6', 'A'), HashSet::from([vec!['v', 'v', 'A']])),
        (('7', '0'), HashSet::from([vec!['>', 'v', 'v', 'v', 'A']])),
        (('7', '1'), HashSet::from([vec!['v', 'v', 'A']])),
        (('7', '2'), HashSet::from([vec!['v', 'v', '>', 'A'], vec!['>', 'v', 'v', 'A']])),
        (('7', '3'), HashSet::from([vec!['v', 'v', '>', '>', 'A'], vec!['>', '>', 'v', 'v', 'A']])),
        (('7', '4'), HashSet::from([vec!['v', 'A']])),
        (('7', '5'), HashSet::from([vec!['v', '>', 'A'], vec!['>', 'v', 'A']])),
        (('7', '6'), HashSet::from([vec!['v', '>', '>', 'A'], vec!['>', '>', 'v', 'A']])),
        (('7', '7'), HashSet::from([vec!['A']])),
        (('7', '8'), HashSet::from([vec!['>', 'A']])),
        (('7', '9'), HashSet::from([vec!['>', '>', 'A']])),
        (('7', 'A'), HashSet::from([vec!['>', '>', 'v', 'v', 'A']])),
        (('8', '0'), HashSet::from([vec!['v', 'v', 'v', 'A']])),
        (('8', '1'), HashSet::from([vec!['<', 'v', 'v', 'A'], vec!['v', 'v', '<', 'A']])),
        (('8', '2'), HashSet::from([vec!['v', 'v', 'A']])),
        (('8', '3'), HashSet::from([vec!['v', 'v', '>', 'A'], vec!['>', 'v', 'v', 'A']])),
        (('8', '4'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('8', '5'), HashSet::from([vec!['v', 'A']])),
        (('8', '6'), HashSet::from([vec!['v', '>', 'A'], vec!['>', 'v', 'A']])),
        (('8', '7'), HashSet::from([vec!['<', 'A']])),
        (('8', '8'), HashSet::from([vec!['A']])),
        (('8', '9'), HashSet::from([vec!['>', 'A']])),
        (('8', 'A'), HashSet::from([vec!['>', 'v', 'v', 'v', 'A'], vec!['v', 'v', 'v', '>', 'A']])),
        (('9', '0'), HashSet::from([vec!['v', 'v', 'v', 'A']])),
        (('9', '1'), HashSet::from([vec!['<', '<', 'v', 'v', 'A'], vec!['v', 'v', '<', '<', 'A']])),
        (('9', '2'), HashSet::from([vec!['<', 'v', 'v', 'A'], vec!['v', 'v', '<', 'A']])),
        (('9', '3'), HashSet::from([vec!['v', 'v', 'A']])),
        (('9', '4'), HashSet::from([vec!['<', '<', 'v', 'A'], vec!['v', '<', '<', 'A']])),
        (('9', '5'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('9', '6'), HashSet::from([vec!['v', 'A']])),
        (('9', '7'), HashSet::from([vec!['<', '<', 'A']])),
        (('9', '8'), HashSet::from([vec!['<', 'A']])),
        (('9', '9'), HashSet::from([vec!['A']])),
        (('9', 'A'), HashSet::from([vec!['v', 'v', 'v', 'A']])),
        (('A', '0'), HashSet::from([vec!['<', 'A']])),
        (('A', '1'), HashSet::from([vec!['^', '<', '<', 'A']])),
        (('A', '2'), HashSet::from([vec!['^', '<', 'A'], vec!['<', '^', 'A']])),
        (('A', '3'), HashSet::from([vec!['^', 'A']])),
        (('A', '4'), HashSet::from([vec!['^', '^', '<', '<', 'A']])),
        (('A', '5'), HashSet::from([vec!['^', '^', '<', 'A'], vec!['<', '^', '^', 'A']])),
        (('A', '6'), HashSet::from([vec!['^', '^', 'A']])),
        (('A', '7'), HashSet::from([vec!['^', '^', '^', '<', '<', 'A']])),
        (('A', '8'), HashSet::from([vec!['^', '^', '^', '<', 'A'], vec!['<', '^', '^', '^', 'A']])),
        (('A', '9'), HashSet::from([vec!['^', '^', '^', 'A']])),
        (('A', 'A'), HashSet::from([vec!['A']])),
    ]);

    #[rustfmt::skip]
    let controller_fastest_paths: HashMap<(char, char), HashSet<Vec<char>>> = HashMap::from([
        (('^', '^'), HashSet::from([vec!['A']])),
        (('^', '>'), HashSet::from([vec!['v', '>', 'A'], vec!['>', 'v', 'A']])),
        (('^', 'v'), HashSet::from([vec!['v', 'A']])),
        (('^', '<'), HashSet::from([vec!['v', '<', 'A']])),
        (('^', 'A'), HashSet::from([vec!['>', 'A']])),
        (('>', '^'), HashSet::from([vec!['<', '^', 'A'], vec!['^', '<', 'A']])),
        (('>', '>'), HashSet::from([vec!['A']])),
        (('>', 'v'), HashSet::from([vec!['<', 'A']])),
        (('>', '<'), HashSet::from([vec!['<', '<', 'A']])),
        (('>', 'A'), HashSet::from([vec!['^', 'A']])),
        (('v', '^'), HashSet::from([vec!['^', 'A']])),
        (('v', '>'), HashSet::from([vec!['>', 'A']])),
        (('v', 'v'), HashSet::from([vec!['A']])),
        (('v', '<'), HashSet::from([vec!['<', 'A']])),
        (('v', 'A'), HashSet::from([vec!['>', '^', 'A'], vec!['^', '>', 'A']])),
        (('<', '^'), HashSet::from([vec!['>', '^', 'A']])),
        (('<', '>'), HashSet::from([vec!['>', '>', 'A']])),
        (('<', 'v'), HashSet::from([vec!['>', 'A']])),
        (('<', '<'), HashSet::from([vec!['A']])),
        (('<', 'A'), HashSet::from([vec!['>', '>', '^', 'A']])),
        (('A', '^'), HashSet::from([vec!['<', 'A']])),
        (('A', '>'), HashSet::from([vec!['v', 'A']])),
        (('A', 'v'), HashSet::from([vec!['<', 'v', 'A'], vec!['v', '<', 'A']])),
        (('A', '<'), HashSet::from([vec!['v', '<', '<', 'A']])),
        (('A', 'A'), HashSet::from([vec!['A']])),
    ]);

    let codes = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| {
            (
                line[..line.len() - 1].parse::<u64>().unwrap(),
                line.chars().into_iter().collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<(u64, Vec<_>)>>();

    let total_complexity = codes
        .into_iter()
        .fold(0u64, |acc, (numerical_value, chars)| {
            let fastest_paths = find_fastest_paths(
                &keypad_fastest_paths,
                &controller_fastest_paths,
                &chars,
                0,
                3,
            );
            let shortest_path = fastest_paths
                .into_iter()
                .sorted_by(|a, b| a.len().cmp(&b.len()))
                .next()
                .unwrap_or(vec![]);
            let complexity = numerical_value * shortest_path.len() as u64;
            acc + complexity
        });

    println!("The sum of the complexities is {total_complexity}");
}
