use aoc_2024::*;
use std::collections::{BTreeSet, HashMap, HashSet};

fn largest_computer_group<'a>(
    current_clique: HashSet<&'a String>,
    current_computer: &'a String,
    computer_graph: &'a HashMap<String, HashSet<String>>,
) -> HashSet<&'a String> {
    let mut mc = current_clique;

    for connected_computer in computer_graph.get(current_computer).unwrap() {
        if !mc.contains(connected_computer)
            && mc.iter().find(|clique_computer| {
                !computer_graph
                    .get(**clique_computer)
                    .unwrap()
                    .contains(connected_computer)
            }) == None
        {
            let mut next_clique = mc.clone();
            next_clique.insert(connected_computer);

            let next_maximal_clique =
                largest_computer_group(next_clique, connected_computer, computer_graph);

            if next_maximal_clique.len() > mc.len() {
                mc = next_maximal_clique;
            }
        }
    }

    mc
}

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

    let maximal_clique = computer_graph
        .iter()
        .fold(HashSet::new(), |acc, (computer, _)| {
            let mc = largest_computer_group(HashSet::from([computer]), computer, &computer_graph);

            if mc.len() > acc.len() {
                return mc;
            } else {
                return acc;
            }
        });

    let mut password = maximal_clique
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .fold(String::new(), |acc, computer| acc + &computer + ",");

    password.remove(password.len() - 1);

    println!("Password: {password}");
}
