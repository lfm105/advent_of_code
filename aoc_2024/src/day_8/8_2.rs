use aoc_2024::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn is_within_bounds(pos: (i32, i32), dim: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < dim.0 && pos.1 >= 0 && pos.1 < dim.1
}

fn main() {
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();

    let mut dim: (i32, i32) = (-1, -1);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    read_file_lines(get_input_file())
        .into_iter()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c != '.' {
                    antennas
                        .entry(c)
                        .or_insert(HashSet::new())
                        .insert((j as i32, i as i32));
                }

                dim.1 = j as i32 + 1;
            });

            dim.0 = i as i32 + 1;
        });

    antennas.into_iter().for_each(|(_, positions)| {
        positions.into_iter().combinations(2).for_each(|pair| {
            let (p1, p2) = (pair[0], pair[1]);
            let p_diff = (p2.0 - p1.0, p2.1 - p1.1);

            let mut antinode = p1;

            while is_within_bounds(antinode, dim) {
                antinodes.insert(antinode);
                antinode = (antinode.0 - p_diff.0, antinode.1 - p_diff.1);
            }

            antinode = p2;

            while is_within_bounds(antinode, dim) {
                antinodes.insert(antinode);
                antinode = (antinode.0 + p_diff.0, antinode.1 + p_diff.1);
            }
        });
    });

    println!("Number of antinodes: {}", antinodes.len());
}
