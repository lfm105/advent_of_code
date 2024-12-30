use aoc_2024::*;
use std::collections::HashSet;

fn is_position_within_bounds(pos: (i32, i32), dim: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn get_adjacent_positions(pos: (i32, i32), dim: (usize, usize)) -> HashSet<(i32, i32)> {
    let mut adj_pos = HashSet::new();

    for (delta_x, delta_y) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        if is_position_within_bounds((pos.0 + delta_x, pos.1 + delta_y), dim) {
            adj_pos.insert((pos.0 + delta_x, pos.1 + delta_y));
        }
    }

    adj_pos
}

fn calculate_paths(starting_pos: (i32, i32), dim: (usize, usize), map: &Vec<Vec<u32>>) -> u32 {
    let mut explored_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut exploration_queue: Vec<(i32, i32)> = vec![starting_pos];
    let mut path_endings: HashSet<(i32, i32)> = HashSet::new();

    while !exploration_queue.is_empty() {
        let curr_pos = exploration_queue[0];
        exploration_queue.remove(0);

        if !explored_positions.contains(&curr_pos) {
            if map[curr_pos.1 as usize][curr_pos.0 as usize] == 9 {
                path_endings.insert(curr_pos);
            } else {
                for adj_pos in get_adjacent_positions(curr_pos, dim) {
                    if map[adj_pos.1 as usize][adj_pos.0 as usize]
                        == map[curr_pos.1 as usize][curr_pos.0 as usize] + 1
                    {
                        exploration_queue.push(adj_pos);
                    }
                }
            }
        }

        explored_positions.insert(curr_pos);
    }

    path_endings.len() as u32
}

fn main() {
    let mut map = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let dim = (map[0].len(), map.len());

    let mut sum: u32 = 0;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if map[y][x] == 0 {
                sum += calculate_paths((x as i32, y as i32), dim, &map);
            }
        }
    }

    println!("Result: {sum}");
}
