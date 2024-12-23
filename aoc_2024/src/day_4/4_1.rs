use aoc_2024::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

fn get_next_letter(curr_letter: char) -> Option<char> {
    match curr_letter {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

fn get_next_pos(curr_pos: (i32, i32), dim: (i32, i32), dir: &Direction) -> Option<(i32, i32)> {
    let delta_x = match dir {
        Direction::N => 0,
        Direction::NE => 1,
        Direction::E => 1,
        Direction::SE => 1,
        Direction::S => 0,
        Direction::SW => -1,
        Direction::W => -1,
        Direction::NW => -1,
    };

    let delta_y = match dir {
        Direction::N => -1,
        Direction::NE => -1,
        Direction::E => 0,
        Direction::SE => 1,
        Direction::S => 1,
        Direction::SW => 1,
        Direction::W => 0,
        Direction::NW => -1,
    };

    let next_pos: (i32, i32) = (curr_pos.0 + delta_x, curr_pos.1 + delta_y);

    if next_pos.0 >= 0 && next_pos.1 >= 0 && next_pos.0 < dim.0 && next_pos.1 < dim.1 {
        return Some(next_pos);
    }

    None
}

fn find_xmas(
    board: &Vec<Vec<char>>,
    curr_pos: (i32, i32),
    dim: (i32, i32),
    dir: Direction,
    curr_letter: char,
) -> bool {
    let next_letter_opt = get_next_letter(curr_letter);

    match next_letter_opt {
        Some(next_letter) => {
            let next_pos_opt = get_next_pos(curr_pos, dim, &dir);

            match next_pos_opt {
                Some(next_pos) => {
                    if board[next_pos.1 as usize][next_pos.0 as usize] != next_letter {
                        return false;
                    }

                    find_xmas(board, next_pos, dim, dir, next_letter)
                }
                None => false,
            }
        }
        None => true,
    }
}

fn main() {
    let board = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim = (board[0].len(), board.len());

    let mut xmas_count = 0;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if board[y][x] == 'X' {
                Direction::iter().for_each(|dir| {
                    xmas_count += find_xmas(
                        &board,
                        (x as i32, y as i32),
                        (dim.0 as i32, dim.1 as i32),
                        dir,
                        'X',
                    ) as i32;
                });
            }
        }
    }

    println!("XMAS count: {xmas_count}");
}
