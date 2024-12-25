use aoc_2024::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn get_direction(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Direction::N),
        '>' => Some(Direction::E),
        'v' => Some(Direction::S),
        '<' => Some(Direction::W),
        _ => None,
    }
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn get_next_pos(curr_pos: &(i32, i32), dim: &(i32, i32), dir: &Direction) -> Option<(i32, i32)> {
    let delta_x = match dir {
        Direction::N => 0,
        Direction::E => 1,
        Direction::S => 0,
        Direction::W => -1,
    };

    let delta_y = match dir {
        Direction::N => -1,
        Direction::E => 0,
        Direction::S => 1,
        Direction::W => 0,
    };

    let next_pos: (i32, i32) = (curr_pos.0 + delta_x, curr_pos.1 + delta_y);

    if next_pos.0 >= 0 && next_pos.1 >= 0 && next_pos.0 < dim.0 && next_pos.1 < dim.1 {
        return Some(next_pos);
    }

    None
}

fn loops(
    starting_pos: &(i32, i32),
    obstruction: (i32, i32),
    dim: &(i32, i32),
    dir: &Direction,
    board: &Vec<Vec<(char, HashSet<Direction>)>>,
) -> bool {
    let mut curr_pos = starting_pos.clone();
    let mut curr_dir = dir.clone();
    let mut board_with_obstruction = board.clone();

    board_with_obstruction[obstruction.1 as usize][obstruction.0 as usize].0 = '#';

    while let Some(next_pos) = get_next_pos(&curr_pos, &dim, &curr_dir) {
        if board_with_obstruction[curr_pos.1 as usize][curr_pos.0 as usize].0 == 'X'
            && board_with_obstruction[curr_pos.1 as usize][curr_pos.0 as usize]
                .1
                .contains(&curr_dir)
        {
            return true;
        }

        board_with_obstruction[curr_pos.1 as usize][curr_pos.0 as usize].0 = 'X';
        board_with_obstruction[curr_pos.1 as usize][curr_pos.0 as usize]
            .1
            .insert(curr_dir);

        if board_with_obstruction[next_pos.1 as usize][next_pos.0 as usize].0 == '#' {
            curr_dir = turn_right(&curr_dir);
        } else {
            curr_pos = next_pos;
        }
    }

    false
}

fn main() {
    let board = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| (c, HashSet::new()))
                .collect::<Vec<(char, HashSet<Direction>)>>()
        })
        .collect::<Vec<Vec<(char, HashSet<Direction>)>>>();

    let dim = (board[0].len(), board.len());

    let mut curr_pos: (i32, i32) = (-1, -1);
    let mut curr_dir: Direction = Direction::N;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if let Some(dir) = get_direction(board[y][x].0) {
                curr_pos = (x as i32, y as i32);
                curr_dir = dir;
                break;
            }
        }
    }

    let mut obstructions: i32 = 0;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if (x as i32, y as i32) != curr_pos && board[y][x].0 != '#' {
                obstructions += loops(
                    &curr_pos,
                    (x as i32, y as i32),
                    &(dim.0 as i32, dim.1 as i32),
                    &curr_dir,
                    &board,
                ) as i32;
            }
        }
    }

    println!("Obstruction count: {}", obstructions);
}
