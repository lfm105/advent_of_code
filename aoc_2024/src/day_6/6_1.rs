use aoc_2024::*;

#[derive(Debug, PartialEq)]
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

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::N => Direction::E,
        Direction::E => Direction::S,
        Direction::S => Direction::W,
        Direction::W => Direction::N,
    }
}

fn get_next_pos(curr_pos: (i32, i32), dim: (i32, i32), dir: &Direction) -> Option<(i32, i32)> {
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

fn main() {
    let mut board = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim = (board[0].len(), board.len());

    let mut curr_pos: (i32, i32) = (-1, -1);
    let mut curr_dir: Direction = Direction::N;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if let Some(dir) = get_direction(board[y][x]) {
                curr_pos = (x as i32, y as i32);
                curr_dir = dir;
                break;
            }
        }
    }

    let mut pos_count = 1;

    while let Some(next_pos) = get_next_pos(curr_pos, (dim.0 as i32, dim.1 as i32), &curr_dir) {
        if board[curr_pos.1 as usize][curr_pos.0 as usize] != 'X'
        {
            pos_count += 1;
            board[curr_pos.1 as usize][curr_pos.0 as usize] = 'X';
        }

        if board[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            curr_dir = turn_right(curr_dir);
        } else {
            curr_pos = next_pos;
        }
    }

    println!("Position count: {pos_count}");
}
