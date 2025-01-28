use aoc_2024::*;
use std::collections::{BTreeMap, HashSet};

fn build_cache(
    controller_fastest_paths: &BTreeMap<(char, char), HashSet<Vec<char>>>,
    cache: &mut BTreeMap<((char, char), u32), u64>,
    depth: u32,
) {
    if depth == 0 {
        return;
    }

    controller_fastest_paths
        .iter()
        .for_each(|((src, target), cfp)| {
            let length_of_fastest_path = cfp.iter().fold(u64::max_value(), |acc, cfp_path| {
                let mut fpp_len = 0u64;

                if cfp_path.is_empty() {
                    return 1; // press 'A'
                }

                fpp_len += cache.get(&(('A', cfp_path[0]), depth + 1)).unwrap_or(
                    &(controller_fastest_paths
                        .get(&('A', cfp_path[0]))
                        .unwrap()
                        .iter()
                        .next()
                        .unwrap()
                        .len() as u64),
                );
                fpp_len += 1;

                for i in 1..cfp_path.len() {
                    fpp_len += cache
                        .get(&((cfp_path[i - 1], cfp_path[i]), depth + 1))
                        .unwrap_or(
                            &(controller_fastest_paths
                                .get(&(cfp_path[i - 1], cfp_path[i]))
                                .unwrap()
                                .iter()
                                .next()
                                .unwrap()
                                .len() as u64),
                        );
                    fpp_len += 1;
                }

                if acc > fpp_len {
                    return fpp_len;
                }

                acc
            });

            cache.insert(((*src, *target), depth), length_of_fastest_path);
        });

    build_cache(controller_fastest_paths, cache, depth - 1);
}

fn main() {
    #[rustfmt::skip]
    let keypad_fastest_paths: BTreeMap<(char, char), HashSet<Vec<char>>> = BTreeMap::from([
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
        (('2', '6'), HashSet::from([vec!['^', '>', 'A'], vec!['>', '^', 'A']])),
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
    let controller_fastest_paths: BTreeMap<(char, char), HashSet<Vec<char>>> = BTreeMap::from([
        (('^', '^'), HashSet::from([vec!['A']])),
        (('^', '>'), HashSet::from([vec!['v', '>'], vec!['>', 'v', 'A']])),
        (('^', 'v'), HashSet::from([vec!['v', 'A']])),
        (('^', '<'), HashSet::from([vec!['v', '<', 'A']])),
        (('^', 'A'), HashSet::from([vec!['>', 'A']])),
        (('>', '^'), HashSet::from([vec!['<', '^'], vec!['^', '<', 'A']])),
        (('>', '>'), HashSet::from([vec!['A']])),
        (('>', 'v'), HashSet::from([vec!['<', 'A']])),
        (('>', '<'), HashSet::from([vec!['<', '<', 'A']])),
        (('>', 'A'), HashSet::from([vec!['^', 'A']])),
        (('v', '^'), HashSet::from([vec!['^', 'A']])),
        (('v', '>'), HashSet::from([vec!['>', 'A']])),
        (('v', 'v'), HashSet::from([vec!['A']])),
        (('v', '<'), HashSet::from([vec!['<', 'A']])),
        (('v', 'A'), HashSet::from([vec!['>', '^'], vec!['^', '>', 'A']])),
        (('<', '^'), HashSet::from([vec!['>', '^', 'A']])),
        (('<', '>'), HashSet::from([vec!['>', '>', 'A']])),
        (('<', 'v'), HashSet::from([vec!['>', 'A']])),
        (('<', '<'), HashSet::from([vec!['A']])),
        (('<', 'A'), HashSet::from([vec!['>', '>', '^', 'A']])),
        (('A', '^'), HashSet::from([vec!['<', 'A']])),
        (('A', '>'), HashSet::from([vec!['v', 'A']])),
        (('A', 'v'), HashSet::from([vec!['<', 'v'], vec!['v', '<', 'A']])),
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

    let robots = 26u32;

    // the path that I have to do from the highest robot to move from A to B
    let mut cache: BTreeMap<((char, char), u32), u64> = controller_fastest_paths
        .iter()
        .map(|((src, target), fps)| {
            (
                ((*src, *target), robots),
                fps.iter().next().unwrap().len() as u64 + 1,
            )
        })
        .collect();

    build_cache(&controller_fastest_paths, &mut cache, robots);

    let total_complexity = codes
        .into_iter()
        .fold(0u64, |acc, (numerical_value, chars)| {
            let mut key_presses = 0;

            for i in 0..chars.len() {
                // for each character in code
                let shortest_paths = match i {
                    0 => keypad_fastest_paths.get(&('A', chars[0])).unwrap(), // if first character, we need to go from A to the current character
                    _ => keypad_fastest_paths.get(&(chars[i - 1], chars[i])).unwrap(),
                }; // else, we need to go from the previous character to the current character

                let shortest_path_len = shortest_paths.iter().fold(u64::max_value(), |acc, fp| {
                    // get all the shortest paths
                    let mut shortest_path_len_2 = 0u64;

                    for i in 0..fp.len() {
                        // for each character in the shortest path
                        match i {
                            0 => {
                                shortest_path_len_2 +=
                                    *cache.get(&(('A', fp[0]), 1)).unwrap() as u64
                            } // if first character, we need to go from A to the current character
                            _ => {
                                shortest_path_len_2 +=
                                    *cache.get(&((fp[i - 1], fp[i]), 1)).unwrap() as u64
                            } // else, we need to go from the previous character to the current character
                        }
                    }

                    if shortest_path_len_2 < acc {
                        // if this path is shorter than the current shortest, update the shortest
                        return shortest_path_len_2;
                    } else {
                        return acc;
                    }
                });

                key_presses += shortest_path_len;
            }

            acc + numerical_value * key_presses
        });

    println!("The sum of the complexities is {total_complexity}");
}
