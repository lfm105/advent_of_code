use aoc_2024::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Position = (i32, i32);

fn is_position_within_bounds(pos: Position, dim: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn get_adj_pos(
    pos: Position,
    corrupted_bytes: &HashSet<Position>,
    dim: (usize, usize),
) -> HashSet<Position> {
    let mut adj_pos = HashSet::new();

    for (delta_x, delta_y) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let p = (pos.0 + delta_x, pos.1 + delta_y);
        if is_position_within_bounds(p, dim) && !corrupted_bytes.contains(&p) {
            adj_pos.insert(p);
        }
    }

    adj_pos
}

fn shortest_path_distance(
    starting_pos: Position,
    goal_pos: Position,
    corrupted_bytes: &HashSet<Position>,
    dim: (usize, usize),
) -> u32 {
    let mut open_set: Vec<Position> = vec![starting_pos];
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut dst: HashMap<Position, u32> = HashMap::new();
    dst.insert(starting_pos, 0u32);

    while !open_set.is_empty() {
        open_set.sort_by(|node_a, node_b| {
            if !dst.contains_key(&node_b) {
                return std::cmp::Ordering::Greater;
            } else if !dst.contains_key(&node_a) {
                return std::cmp::Ordering::Less;
            } else {
                return dst.get(&node_a).unwrap().cmp(dst.get(&node_b).unwrap());
            }
        });

        let curr_pos = open_set.remove(0);

        if curr_pos == goal_pos {
            return *dst.get(&curr_pos).unwrap();
        }

        for adj_pos in get_adj_pos(curr_pos, corrupted_bytes, dim) {
            if !dst.contains_key(&adj_pos)
                || (dst.get(&curr_pos).unwrap() + 1 < *dst.get(&adj_pos).unwrap())
            {
                dst.insert(adj_pos, dst.get(&curr_pos).unwrap() + 1);
                came_from.insert(adj_pos, curr_pos);

                if open_set.iter().find(|p| **p == adj_pos) == None {
                    open_set.push(adj_pos);
                }
            }
        }
    }

    panic!("Could not find path!");
}

fn main() {
    let number_of_bytes = 1024usize;

    let corrupted_bytes: HashSet<Position> = read_file_lines(get_input_file())
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| *idx < number_of_bytes)
        .map(|(_, line)| {
            let mut iter = line.split(",");
            let x = iter.next().unwrap().parse::<i32>().unwrap();
            let y = iter.next().unwrap().parse::<i32>().unwrap();

            (x, y)
        })
        .collect();

    let dim = (71usize, 71usize);
    let starting_pos = (0i32, 0i32);
    let goal_pos = (70i32, 70i32);

    let distance = shortest_path_distance(starting_pos, goal_pos, &corrupted_bytes, dim);

    println!("Shortest path: {distance}");
}
