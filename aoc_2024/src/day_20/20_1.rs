use aoc_2024::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

fn is_position_within_bounds(pos: Position, dim: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn get_adj_pos(
    pos: Position,
    racetrack: &Vec<Vec<char>>,
    dim: (usize, usize),
) -> HashSet<Position> {
    let mut adj_pos = HashSet::new();

    for (delta_x, delta_y) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let p = (pos.0 + delta_x, pos.1 + delta_y);
        if is_position_within_bounds(p, dim) && (racetrack[p.1 as usize][p.0 as usize] == '.') {
            adj_pos.insert(p);
        }
    }

    adj_pos
}

fn distance_from_start(
    starting_pos: Position,
    goal_pos: Position,
    racetrack: &Vec<Vec<char>>,
    dim: (usize, usize),
) -> HashMap<Position, u32> {
    let mut current_pos: Option<Position> = Some(starting_pos);
    let mut visited_pos: HashSet<Position> = HashSet::new();
    let mut dst: HashMap<Position, u32> = HashMap::new();
    let mut curr_dst = 0u32;

    while let Some(curr_pos) = current_pos {
        visited_pos.insert(curr_pos);
        dst.insert(curr_pos, curr_dst);
        curr_dst += 1;

        if curr_pos != goal_pos {
            for ap in get_adj_pos(curr_pos, racetrack, dim) {
                if !visited_pos.contains(&ap) {
                    current_pos = Some(ap);
                    break;
                }
            }
        } else {
            current_pos = None;
        }
    }

    dst
}

fn main() {
    let mut starting_pos: Option<Position> = None;
    let mut goal_pos: Option<Position> = None;
    let mut walls: HashSet<Position> = HashSet::new();

    let racetrack = read_file_lines(get_input_file())
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        starting_pos = Some((col as i32, row as i32));
                        return '.';
                    } else if c == 'E' {
                        goal_pos = Some((col as i32, row as i32));
                        return '.';
                    } else if c == '#' {
                        walls.insert((col as i32, row as i32));
                    } else if c != '.' {
                        panic!("Invalid input!");
                    }

                    c
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    if starting_pos == None {
        panic!("No starting position!");
    }

    if goal_pos == None {
        panic!("No goal position!");
    }

    let dim: (usize, usize) = (racetrack.len(), racetrack[0].len());

    let dst_from_start =
        distance_from_start(starting_pos.unwrap(), goal_pos.unwrap(), &racetrack, dim);

    let mut savings: HashMap<(Position, Position), i32> = HashMap::new();

    walls.into_iter().for_each(|wall| {
        let adj_positions = get_adj_pos(wall, &racetrack, dim);

        adj_positions.iter().permutations(2).for_each(|positions| {
            let (p1, p2) = (positions[0], positions[1]);

            savings.insert(
                (*p1, *p2),
                *dst_from_start.get(p2).unwrap() as i32
                    - *dst_from_start.get(p1).unwrap() as i32
                    - 2,
            );
        });
    });

    let num_cheats = savings.into_iter().fold(0u32, |acc, (_, v)| {
        if v >= 100 {
            return acc + 1;
        }

        return acc;
    });

    println!("There are {num_cheats} cheats that save at least 100 picoseconds");
}
