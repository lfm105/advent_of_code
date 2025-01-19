use aoc_2024::*;
use std::cmp::Eq;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
    let mut computer_graph: HashMap<String, HashSet<String>> = HashMap::new();

    read_file_lines(get_input_file())
        .into_iter()
        .for_each(|line| {
            let computers: Vec<&str> = line.split('-').collect();

            let computer_1 = computers[0];
            let computer_2 = computers[1];

            if computer_graph.contains_key(computer_1) {
                computer_graph
                    .get_mut(computer_1)
                    .unwrap()
                    .insert(computer_2.to_owned());
            } else {
                computer_graph.insert(
                    computer_1.to_owned(),
                    HashSet::from([computer_2.to_owned()]),
                );
            }

            if computer_graph.contains_key(computer_2) {
                computer_graph
                    .get_mut(computer_2)
                    .unwrap()
                    .insert(computer_1.to_owned());
            } else {
                computer_graph.insert(
                    computer_2.to_owned(),
                    HashSet::from([computer_1.to_owned()]),
                );
            }
        });

    let mut three_computer_groups: HashSet<BTreeSet<String>> = HashSet::new();

    computer_graph
        .iter()
        .for_each(|(computer, connected_computers)| {
            connected_computers.iter().for_each(|connected_computer| {
                computer_graph
                    .get(connected_computer)
                    .unwrap()
                    .iter()
                    .for_each(|other_connected_computer| {
                        if computer_graph
                            .get(other_connected_computer)
                            .unwrap()
                            .contains(computer)
                        {
                            three_computer_groups.insert(BTreeSet::from([
                                computer.clone(),
                                connected_computer.clone(),
                                other_connected_computer.clone(),
                            ]));
                        }
                    });
            });
        });

    let t_computer_sets = three_computer_groups
        .into_iter()
        .fold(0u32, |acc, computer_group| {
            if computer_group
                .into_iter()
                .find(|computer| computer.starts_with("t"))
                != None
            {
                return acc + 1;
            } else {
                return acc;
            }
        });

    println!("Number of three computer groups with any name starting with 't': {t_computer_sets}");
}
