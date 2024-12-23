use aoc_2024::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines = read_file_lines(get_input_file());

    let mut dep_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    lines.into_iter().for_each(|l| {
        if l.contains("|") {
            let mut split_line = l.split("|");
            let (x, y) = (
                split_line.next().unwrap().parse::<i32>().unwrap(),
                split_line.next().unwrap().parse::<i32>().unwrap(),
            );

            if dep_map.get(&y) == None {
                dep_map.insert(y, HashSet::new());
            }
            dep_map.get_mut(&y).unwrap().insert(x);
        } else if l.contains(",") {
            updates.push(
                l.split(",")
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    });

    let mut result = 0;

    updates.into_iter().for_each(|update| {
        let valid = update.iter().enumerate().all(|(idx, el)| {
            update[idx..].iter().all(|number| {
                let deps_opt = dep_map.get(el);

                deps_opt == None || !deps_opt.unwrap().contains(number)
            })
        });

        if valid {
            result += update[update.len() / 2];
        }
    });

    println!("Result: {result}");
}
