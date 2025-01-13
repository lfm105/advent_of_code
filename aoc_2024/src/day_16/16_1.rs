use aoc_2024::*;
use std::collections::{HashMap, HashSet};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Position = (i32, i32);

type Node = (Position, Direction);

fn rotate_direction_clockwise(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn rotate_direction_counter_clockwise(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::W,
        Direction::W => Direction::S,
        Direction::S => Direction::E,
        Direction::E => Direction::N,
    }
}

fn is_position_within_bounds(pos: Position, dim: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < dim.0 && pos.1 >= 0 && pos.1 < dim.1
}

fn get_next_pos(pos: (i32, i32), dir: Direction, dim: (i32, i32)) -> Option<(i32, i32)> {
    let delta_x = match dir {
        Direction::E => 1,
        Direction::W => -1,
        _ => 0,
    };

    let delta_y = match dir {
        Direction::N => -1,
        Direction::S => 1,
        _ => 0,
    };

    let next_pos = (pos.0 + delta_x, pos.1 + delta_y);

    if !is_position_within_bounds(next_pos, dim) {
        return None;
    } else {
        return Some(next_pos);
    }
}

fn euclidean_distance(pos_a: Position, pos_b: Position) -> i64 {
    (((pos_b.0 - pos_a.0) as f32).powf(2f32) + ((pos_b.1 - pos_a.1) as f32).powf(2f32)).sqrt()
        as i64
}

fn manhattan_distance(pos_a: Position, pos_b: Position) -> i64 {
    return (pos_b.0 - pos_a.0).abs() as i64 + (pos_b.1 - pos_a.1).abs() as i64;
}

fn get_neighbouring_nodes(node: Node, maze: &Vec<Vec<char>>, dim: (i32, i32)) -> HashSet<Node> {
    let mut neighbours: HashSet<Node> = HashSet::new();

    let counter_clockwise_dir = rotate_direction_counter_clockwise(node.1);

    if let Some(next_counter_clockwise_pos) = get_next_pos(node.0, counter_clockwise_dir, dim) {
        if maze[next_counter_clockwise_pos.1 as usize][next_counter_clockwise_pos.0 as usize] == '.'
        {
            neighbours.insert((next_counter_clockwise_pos, counter_clockwise_dir));
        }
    }

    if let Some(next_forward_pos) = get_next_pos(node.0, node.1, dim) {
        if maze[next_forward_pos.1 as usize][next_forward_pos.0 as usize] == '.' {
            neighbours.insert((next_forward_pos, node.1));
        }
    }

    let clockwise_dir = rotate_direction_clockwise(node.1);

    if let Some(next_clockwise_pos) = get_next_pos(node.0, clockwise_dir, dim) {
        if maze[next_clockwise_pos.1 as usize][next_clockwise_pos.0 as usize] == '.' {
            neighbours.insert((next_clockwise_pos, clockwise_dir));
        }
    }

    neighbours
}

fn calculate_path_score(came_from: &HashMap<Node, Node>, current_node: Node) -> i64 {
    let mut score = 0i64;

    let mut curr: &Node = &current_node;

    while came_from.contains_key(&curr) {
        score += 1;

        if came_from.get(&curr).unwrap().1 != curr.1 {
            score += 1000;
        }

        curr = came_from.get(&curr).unwrap();
    }

    score
}

fn a_star(
    starting_pos: Position,
    goal_pos: Position,
    heuristic: fn(Position, Position) -> i64,
    maze: &Vec<Vec<char>>,
    dim: (i32, i32),
) -> i64 {
    let mut open_set: Vec<Node> = vec![(starting_pos, Direction::E)];

    let mut came_from: HashMap<Node, Node> = HashMap::new();

    let mut lowest_scores: HashMap<Node, i64> = HashMap::new();
    lowest_scores.insert((starting_pos, Direction::E), 0);

    let mut possible_lowest_score: HashMap<Node, i64> = HashMap::new();
    possible_lowest_score.insert(
        (starting_pos, Direction::E),
        heuristic(starting_pos, goal_pos),
    );

    while !open_set.is_empty() {
        open_set.sort_by(|node_a, node_b| {
            if !lowest_scores.contains_key(&node_b) {
                return std::cmp::Ordering::Greater;
            } else if !lowest_scores.contains_key(&node_a) {
                return std::cmp::Ordering::Less;
            } else {
                return lowest_scores
                    .get(&node_a)
                    .unwrap()
                    .cmp(lowest_scores.get(&node_b).unwrap());
            }
        });

        let current_node: Node = open_set.remove(0);

        if current_node.0 == goal_pos {
            return calculate_path_score(&came_from, current_node);
        }

        for neighbour in get_neighbouring_nodes(current_node, maze, dim) {
            let mut points: i64 = 1;

            if neighbour.1 != current_node.1 {
                points += 1000;
            }

            points += lowest_scores.get(&current_node).unwrap();

            if !lowest_scores.contains_key(&neighbour) || (points < *lowest_scores.get(&neighbour).unwrap()) {
                came_from.insert(neighbour, current_node);
                lowest_scores.insert(neighbour, points);
                possible_lowest_score.insert(neighbour, points + heuristic(current_node.0, goal_pos));
                open_set.push(neighbour);
            }
        }
    }

    panic!("Could not find goal position!");
}

fn main() {
    let mut starting_pos: Option<Position> = None;
    let mut goal_pos: Option<Position> = None;

    let maze = read_file_lines(get_input_file())
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
                    } else if c != '#' && c != '.' {
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

    let (starting_pos, goal_pos) = (starting_pos.unwrap(), goal_pos.unwrap());
    let dim: (i32, i32) = (maze[0].len() as i32, maze.len() as i32);

    let minimum_score = a_star(starting_pos, goal_pos, manhattan_distance, &maze, dim);

    println!("Minimum score: {minimum_score}");
}
