use aoc_2024::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

fn get_neighbouring_nodes(node: Node, maze: &Vec<Vec<char>>, dim: (i32, i32)) -> HashSet<Node> {
    let mut neighbours: HashSet<Node> = HashSet::new();

    neighbours.insert((node.0, rotate_direction_counter_clockwise(node.1)));

    if let Some(next_forward_pos) = get_next_pos(node.0, node.1, dim) {
        if maze[next_forward_pos.1 as usize][next_forward_pos.0 as usize] == '.' {
            neighbours.insert((next_forward_pos, node.1));
        }
    }

    neighbours.insert((node.0, rotate_direction_clockwise(node.1)));

    neighbours
}

fn get_paths_positions(
    came_from: &HashMap<Node, HashSet<Node>>,
    lowest_scores: &HashMap<Node, i64>,
    end_nodes: HashSet<Node>,
    best_path_score: i64,
) -> HashSet<Position> {
    let mut positions: HashSet<Position> = HashSet::new();
    let mut explored_nodes: HashSet<Node> = HashSet::new();
    let mut lowest_scores_rev: HashMap<Node, i64> = HashMap::new();
    let mut exploration_queue: Vec<Node> = vec![];

    end_nodes.into_iter().for_each(|n| {
        exploration_queue.push(n);
        lowest_scores_rev.insert(n, 0);
    });

    while !exploration_queue.is_empty() {
        let curr = exploration_queue.remove(0);

        if !explored_nodes.contains(&curr) {
            explored_nodes.insert(curr);

            if lowest_scores_rev.get(&curr).unwrap() + lowest_scores.get(&curr).unwrap()
                == best_path_score
            {
                positions.insert(curr.0);

                if came_from.contains_key(&curr) {
                    came_from.get(&curr).unwrap().iter().for_each(|n| {
                        exploration_queue.push(*n);

                        let points: i64;

                        if n.0 != curr.0 {
                            points = 1;
                        } else {
                            points = 1000;
                        }

                        lowest_scores_rev
                            .insert(*n, lowest_scores_rev.get(&curr).unwrap() + points);
                    });
                }
            }
        }
    }

    positions
}

fn djikstra(
    starting_pos: Position,
    goal_pos: Position,
    maze: &Vec<Vec<char>>,
    dim: (i32, i32),
) -> HashSet<Position> {
    let mut open_set: Vec<Node> = vec![(starting_pos, Direction::E)];

    let mut lowest_scores: HashMap<Node, i64> = HashMap::new();

    let mut came_from: HashMap<Node, HashSet<Node>> = HashMap::new();
    lowest_scores.insert((starting_pos, Direction::E), 0);

    let mut best_path_score = i64::max_value();
    let mut last_nodes = HashSet::<Node>::new();

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
            if *lowest_scores.get(&current_node).unwrap() < best_path_score {
                best_path_score = *lowest_scores.get(&current_node).unwrap();
                last_nodes.clear();
            }

            if *lowest_scores.get(&current_node).unwrap() <= best_path_score {
                last_nodes.insert(current_node);
            }
        }

        for neighbour in get_neighbouring_nodes(current_node, maze, dim) {
            let mut points = 0i64;

            if neighbour.0 != current_node.0 {
                points = 1;
            } else if neighbour.1 != current_node.1 {
                points = 1000;
            }

            points += lowest_scores.get(&current_node).unwrap();

            if !lowest_scores.contains_key(&neighbour)
                || (points <= *lowest_scores.get(&neighbour).unwrap())
            {
                if !came_from.contains_key(&neighbour)
                    || (points < *lowest_scores.get(&neighbour).unwrap())
                {
                    came_from.insert(neighbour, HashSet::new());
                }

                came_from.get_mut(&neighbour).unwrap().insert(current_node);

                lowest_scores.insert(neighbour, points);

                if open_set.iter().find(|n| *n == &neighbour) == None {
                    open_set.push(neighbour);
                }
            }
        }
    }

    let mut positions: HashSet<Position> = HashSet::new();

    get_paths_positions(&came_from, &lowest_scores, last_nodes, best_path_score)
        .into_iter()
        .for_each(|p| {
            positions.insert(p);
        });

    positions
}

fn main() {
    let mut starting_pos: Option<Position> = None;
    let mut goal_pos: Option<Position> = None;

    let mut maze = read_file_lines(get_input_file())
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

    let best_paths_positions = djikstra(starting_pos, goal_pos, &maze, dim);

    let number_of_best_path_positions = best_paths_positions.len();

    best_paths_positions.into_iter().for_each(|pos| {
        maze[pos.1 as usize][pos.0 as usize] = 'O';
    });

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            print!("{}", maze[y as usize][x as usize])
        }
        println!()
    }

    println!("Number of positions visited: {number_of_best_path_positions}");
}
