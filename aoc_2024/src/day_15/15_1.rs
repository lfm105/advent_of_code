use aoc_2024::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
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

fn get_next_pos(pos: (i32, i32), direction: Direction, dim: (i32, i32)) -> Option<(i32, i32)> {
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

    let next_pos = (pos.0 + delta_x, pos.1 + delta_y);

    if next_pos.0 < 0 || next_pos.0 >= dim.0 || next_pos.1 < 0 || next_pos.1 >= dim.1 {
        return None;
    } else {
        return Some(next_pos);
    }
}

fn main() {
    let file_lines = read_file_lines(get_input_file());

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<Direction> = Vec::new();

    file_lines.into_iter().for_each(|line| {
        if line.starts_with("#") {
            map.push(line.chars().collect::<Vec<_>>());
        } else {
            line.chars().for_each(|c| movements.push(get_direction(c)));
        }
    });

    let mut robot_pos = (0i32, 0i32);

    let dim = (map[0].len(), map.len());

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if map[y][x] == '@' {
                robot_pos = (x as i32, y as i32);
                break;
            }
        }
    }
    
    movements.into_iter().for_each(|m| {
        if let Some(next_robot_pos) = get_next_pos(robot_pos, m, (dim.0 as i32, dim.1 as i32)) {
            if map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] == '.' {
                map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] = '@';
                map[robot_pos.1 as usize][robot_pos.0 as usize] = '.';
                robot_pos = next_robot_pos;
            } else if map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] == 'O' {
                let mut last_box_pos = next_robot_pos;

                while let Some(next_box_pos) =
                    get_next_pos(last_box_pos, m, (dim.0 as i32, dim.1 as i32))
                {
                    if map[next_box_pos.1 as usize][next_box_pos.0 as usize] == 'O' {
                        last_box_pos = next_box_pos;
                    } else {
                        break;
                    }
                }

                if let Some(last_box_next_pos) =
                    get_next_pos(last_box_pos, m, (dim.0 as i32, dim.1 as i32))
                {
                    if map[last_box_next_pos.1 as usize][last_box_next_pos.0 as usize] == '.' {
                        map[last_box_next_pos.1 as usize][last_box_next_pos.0 as usize] = 'O';
                        map[next_robot_pos.1 as usize][next_robot_pos.0 as usize] = '@';
                        map[robot_pos.1 as usize][robot_pos.0 as usize] = '.';
                        robot_pos = next_robot_pos;
                    }
                }
            }
        }
    });

    let mut gps_coordinates = 0i64;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if map[y][x] == 'O' {
                gps_coordinates += x as i64;
                gps_coordinates += 100 * y as i64;
            }
        }
    }

    println!("Sum of GPS coordinates: {gps_coordinates}");
}
