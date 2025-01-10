use aoc_2024::*;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Robot,
    Box(i32, i32), // left side pos
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Direction::N => write!(f, "^"),
            Direction::E => write!(f, ">"),
            Direction::S => write!(f, "v"),
            Direction::W => write!(f, "<"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Robot => write!(f, "@"),
            Tile::Box(_, _) => write!(f, "|"),
        }
    }
}

fn get_direction(c: char) -> Direction {
    match c {
        '^' => Direction::N,
        '>' => Direction::E,
        'v' => Direction::S,
        '<' => Direction::W,
        _ => panic!(),
    }
}

fn get_next_pos(
    pos: (i32, i32),
    direction: Direction,
    step: i32,
    dim: (i32, i32),
) -> Option<(i32, i32)> {
    let delta_x = match direction {
        Direction::E => 1,
        Direction::W => -1,
        _ => 0,
    };

    let delta_y = match direction {
        Direction::N => -1,
        Direction::S => 1,
        _ => 0,
    };

    let next_pos = (pos.0 + delta_x * step, pos.1 + delta_y * step);

    if next_pos.0 < 0 || next_pos.0 >= dim.0 || next_pos.1 < 0 || next_pos.1 >= dim.1 {
        return None;
    } else {
        return Some(next_pos);
    }
}

fn can_be_moved(b: Tile, dir: Direction, map: &Vec<Vec<Tile>>, dim: (i32, i32)) -> bool {
    match b {
        Tile::Box(box_x, box_y) => match dir {
            Direction::N | Direction::S => {
                if let (Some(next_left_box_pos), Some(next_right_box_pos)) = (
                    get_next_pos((box_x, box_y), dir, 1, dim),
                    get_next_pos((box_x + 1, box_y), dir, 1, dim),
                ) {
                    let next_box_tiles = (
                        map[next_left_box_pos.1 as usize][next_left_box_pos.0 as usize],
                        map[next_right_box_pos.1 as usize][next_right_box_pos.0 as usize],
                    );

                    match next_box_tiles {
                        (Tile::Empty, Tile::Empty) => {
                            return true;
                        }
                        (Tile::Wall, _) | (_, Tile::Wall) => {
                            return false;
                        }
                        (Tile::Box(left_box_x, left_box_y), Tile::Empty) => {
                            return can_be_moved(Tile::Box(left_box_x, left_box_y), dir, map, dim)
                        }
                        (Tile::Empty, Tile::Box(right_box_x, right_box_y)) => {
                            return can_be_moved(Tile::Box(right_box_x, right_box_y), dir, map, dim)
                        }
                        (
                            Tile::Box(left_box_x, left_box_y),
                            Tile::Box(right_box_x, right_box_y),
                        ) => {
                            return can_be_moved(Tile::Box(left_box_x, left_box_y), dir, map, dim)
                                && can_be_moved(Tile::Box(right_box_x, right_box_y), dir, map, dim)
                        }
                        _ => panic!(),
                    }
                } else {
                    return false;
                }
            }
            Direction::W => {
                if let Some(next_box_pos) = get_next_pos((box_x, box_y), dir, 1, dim) {
                    let next_box_tile = map[next_box_pos.1 as usize][next_box_pos.0 as usize];

                    match next_box_tile {
                        Tile::Empty => return true,
                        Tile::Wall => return false,
                        Tile::Box(west_box_x, west_box_y) => {
                            return can_be_moved(Tile::Box(west_box_x, west_box_y), dir, map, dim)
                        }
                        _ => panic!(),
                    }
                }

                return false;
            }
            Direction::E => {
                if let Some(next_box_pos) = get_next_pos((box_x, box_y), dir, 2, dim) {
                    let next_box_tile = map[next_box_pos.1 as usize][next_box_pos.0 as usize];

                    match next_box_tile {
                        Tile::Empty => return true,
                        Tile::Wall => return false,
                        Tile::Box(west_box_x, west_box_y) => {
                            return can_be_moved(Tile::Box(west_box_x, west_box_y), dir, map, dim)
                        }
                        _ => panic!(),
                    }
                }

                return false;
            }
        },
        _ => false,
    }
}

fn move_box(b: Tile, dir: Direction, map: &mut Vec<Vec<Tile>>, dim: (i32, i32)) {
    match b {
        Tile::Box(box_x, box_y) => match dir {
            Direction::N | Direction::S => {
                if let (Some(next_left_box_pos), Some(next_right_box_pos)) = (
                    get_next_pos((box_x, box_y), dir, 1, dim),
                    get_next_pos((box_x + 1, box_y), dir, 1, dim),
                ) {
                    let next_box_tiles = (
                        map[next_left_box_pos.1 as usize][next_left_box_pos.0 as usize],
                        map[next_right_box_pos.1 as usize][next_right_box_pos.0 as usize],
                    );

                    if let Tile::Box(_, _) = next_box_tiles.0 {
                        if next_box_tiles.0 != next_box_tiles.1 {
                            move_box(next_box_tiles.0, dir, map, dim);
                        }
                    }

                    if let Tile::Box(_, _) = next_box_tiles.1 {
                        move_box(next_box_tiles.1, dir, map, dim);
                    }

                    map[next_left_box_pos.1 as usize][next_left_box_pos.0 as usize] =
                        Tile::Box(next_left_box_pos.0, next_left_box_pos.1);
                    map[next_right_box_pos.1 as usize][next_right_box_pos.0 as usize] =
                        Tile::Box(next_left_box_pos.0, next_left_box_pos.1);
                    map[box_y as usize][box_x as usize] = Tile::Empty;
                    map[box_y as usize][box_x as usize + 1] = Tile::Empty;
                }
            }
            Direction::W => {
                if let Some(next_box_pos) = get_next_pos((box_x, box_y), dir, 1, dim) {
                    let next_box_tile = map[next_box_pos.1 as usize][next_box_pos.0 as usize];

                    if let Tile::Box(_, _) = next_box_tile {
                        move_box(next_box_tile, dir, map, dim);
                    }

                    map[next_box_pos.1 as usize][next_box_pos.0 as usize] =
                        Tile::Box(next_box_pos.0, next_box_pos.1);
                    map[next_box_pos.1 as usize][next_box_pos.0 as usize + 1] =
                        Tile::Box(next_box_pos.0, next_box_pos.1);
                    map[box_y as usize][box_x as usize + 1] = Tile::Empty;
                }
            }
            Direction::E => {
                if let Some(next_box_pos) = get_next_pos((box_x, box_y), dir, 1, dim) {
                    let possible_east_box_pos = get_next_pos(next_box_pos, dir, 1, dim).unwrap();
                    let possible_east_box_tile =
                        map[possible_east_box_pos.1 as usize][possible_east_box_pos.0 as usize];

                    if let Tile::Box(_, _) = possible_east_box_tile {
                        move_box(possible_east_box_tile, dir, map, dim);
                    }

                    map[next_box_pos.1 as usize][next_box_pos.0 as usize] =
                        Tile::Box(next_box_pos.0, next_box_pos.1);
                    map[next_box_pos.1 as usize][next_box_pos.0 as usize + 1] =
                        Tile::Box(next_box_pos.0, next_box_pos.1);
                    map[box_y as usize][box_x as usize] = Tile::Empty;
                }
            }
        },
        _ => panic!(),
    }
}

fn main() {
    let file_lines = read_file_lines(get_input_file());

    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut movements: Vec<Direction> = Vec::new();

    let mut robot_pos = (0i32, 0i32);

    file_lines.into_iter().enumerate().for_each(|(row, line)| {
        if line.starts_with("#") {
            let mut new_line = Vec::<Tile>::new();

            line.chars().enumerate().for_each(|(col, c)| match c {
                '.' => {
                    new_line.push(Tile::Empty);
                    new_line.push(Tile::Empty);
                }
                'O' => {
                    new_line.push(Tile::Box(col as i32 * 2, row as i32));
                    new_line.push(Tile::Box(col as i32 * 2, row as i32));
                }
                '@' => {
                    new_line.push(Tile::Robot);
                    new_line.push(Tile::Empty);
                    robot_pos = (col as i32 * 2, row as i32);
                }
                '#' => {
                    new_line.push(Tile::Wall);
                    new_line.push(Tile::Wall);
                }
                _ => panic!(),
            });

            map.push(new_line);
        } else {
            line.chars().for_each(|c| movements.push(get_direction(c)));
        }
    });

    let dim = (map[0].len(), map.len());

    movements.into_iter().for_each(|m| {
        if let Some(next_robot_pos) = get_next_pos(robot_pos, m, 1, (dim.0 as i32, dim.1 as i32)) {
            match map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] {
                Tile::Empty => {
                    map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] = Tile::Robot;
                    map[robot_pos.1 as usize][robot_pos.0 as usize] = Tile::Empty;
                    robot_pos = next_robot_pos;
                }
                Tile::Box(box_x, box_y) => {
                    if can_be_moved(
                        Tile::Box(box_x, box_y),
                        m,
                        &map,
                        (dim.0 as i32, dim.1 as i32),
                    ) {
                        move_box(
                            Tile::Box(box_x, box_y),
                            m,
                            &mut map,
                            (dim.0 as i32, dim.1 as i32),
                        );
                        map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] = Tile::Robot;
                        map[robot_pos.1 as usize][robot_pos.0 as usize] = Tile::Empty;
                        robot_pos = next_robot_pos;
                    }
                }
                _ => {}
            }
        }
    });
    
    let gps_coordinates: i64 = map.into_iter().fold(0i64, |sum, line| {
        sum + line
            .into_iter()
            .step_by(2)
            .fold(0i64, |mut col_sum, tile| {
                if let Tile::Box(box_x, box_y) = tile {
                    col_sum += box_x as i64;
                    col_sum += 100 * box_y as i64;
                }

                col_sum
            })
    });

    println!("Sum of GPS coordinates: {gps_coordinates}");
}
