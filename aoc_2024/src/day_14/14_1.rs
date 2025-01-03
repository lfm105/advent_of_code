use aoc_2024::*;
use regex::Regex;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Robot = ((i32, i32), (i32, i32)); // (pos), (speed)

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
enum Quadrant {
    NE,
    SE,
    SW,
    NW,
}

fn get_quadrant(pos: (i32, i32), dim: (i32, i32)) -> Option<Quadrant> {
    if (dim.0 % 2 == 1 && pos.0 == dim.0 / 2) || (dim.1 % 2 == 1 && pos.1 == dim.1 / 2) {
        return None;
    } else if pos.0 < dim.0 / 2 {
        if pos.1 < dim.1 / 2 {
            return Some(Quadrant::NW);
        } else {
            return Some(Quadrant::SW);
        }
    } else {
        if pos.1 < dim.1 / 2 {
            return Some(Quadrant::NE);
        } else {
            return Some(Quadrant::SE);
        }
    }
}

fn calculate_next_position(curr_pos: (i32, i32), speed: (i32, i32), dim: (i32, i32)) -> (i32, i32) {
    let mut new_pos = (
        (curr_pos.0 + speed.0) % dim.0,
        (curr_pos.1 + speed.1) % dim.1,
    );

    if new_pos.0 < 0 {
        new_pos.0 += dim.0;
    }

    if new_pos.1 < 0 {
        new_pos.1 += dim.1;
    }

    new_pos
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let re =
        Regex::new(r"p=([0-9]{1,}),([0-9]{1,}) v=([-]{0,1}[0-9]{1,}),([-]{0,1}[0-9]{1,})").unwrap();

    let (mut width, mut height): (i32, i32) = (1, 1);

    let robots = input_lines
        .into_iter()
        .map(|line| {
            let mut robot: Robot = ((0, 0), (0, 0));

            for (_, [pos_x, pos_y, speed_x, speed_y]) in
                re.captures_iter(&line).map(|cc| cc.extract())
            {
                robot.0 .0 = pos_x.parse::<i32>().unwrap();
                robot.0 .1 = pos_y.parse::<i32>().unwrap();
                robot.1 .0 = speed_x.parse::<i32>().unwrap();
                robot.1 .1 = speed_y.parse::<i32>().unwrap();

                if robot.0 .0 + 1 > width {
                    width = robot.0 .0 + 1;
                }

                if robot.0 .1 + 1 > height {
                    height = robot.0 .1 + 1;
                }
            }

            robot
        })
        .collect::<Vec<Robot>>();

    let mut robot_map: HashMap<Quadrant, Vec<Robot>> = HashMap::new();

    Quadrant::iter().for_each(|q| {
        robot_map.insert(q, vec![]);
    });

    robots.into_iter().for_each(|r| {
        let mut robot_pos = r.0;

        for _ in 0..100 {
            robot_pos = calculate_next_position(robot_pos, r.1, (width, height));
        }

        if let Some(quadrant) = get_quadrant(robot_pos, (width, height)) {
            robot_map.get_mut(&quadrant).unwrap().push(r);
        }
    });

    let mut safety_score = 1usize;

    Quadrant::iter().for_each(|q| {
        safety_score *= robot_map.get(&q).unwrap().len();
    });

    println!("Safety score: {safety_score}");
}
