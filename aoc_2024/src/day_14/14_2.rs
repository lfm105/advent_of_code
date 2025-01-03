use aoc_2024::*;
use regex::Regex;

type Robot = ((i32, i32), (i32, i32)); // (pos), (speed)

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

fn dist_to_center(pos: (i32, i32), dim: (i32, i32)) -> i32 {
    let center = (dim.0 / 2, dim.1 / 2);

    (((pos.0 - center.0) as f32).powf(2f32) + ((pos.1 - center.1) as f32).powf(2f32)).sqrt() as i32
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let re =
        Regex::new(r"p=([0-9]{1,}),([0-9]{1,}) v=([-]{0,1}[0-9]{1,}),([-]{0,1}[0-9]{1,})").unwrap();

    let (mut width, mut height): (i32, i32) = (1, 1);

    let mut robots = input_lines
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

    let mut robot_map: Vec<Vec<i32>> = Vec::new();

    for _ in 0..height {
        robot_map.push(vec![]);
    }

    for y in 0..height {
        for _ in 0..width {
            robot_map[y as usize].push(0);
        }
    }

    robots
        .iter()
        .for_each(|r| robot_map[r.0 .1 as usize][r.0 .0 as usize] += 1);

    let mut robot_maps: Vec<(u32, i64, Vec<Vec<i32>>)> = Vec::new();

    for i in 0..10000 {
        let map_dist_sum = robots.iter().fold(0i64, |acc, r| {
            acc + dist_to_center(r.0, (width, height)) as i64
        });

        robot_maps.push((i, map_dist_sum, robot_map.clone()));

        robots.iter_mut().for_each(|r| {
            robot_map[r.0 .1 as usize][r.0 .0 as usize] -= 1;

            r.0 = calculate_next_position(r.0, r.1, (width, height));

            robot_map[r.0 .1 as usize][r.0 .0 as usize] += 1;
        });
    }

    robot_maps.sort_by(|m1, m2| {
        m1.1.cmp(&m2.1)
    });
    
    println!("Most likely time: {} seconds", robot_maps[0].0);
}
