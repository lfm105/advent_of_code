use aoc_2024::*;
use std::collections::HashSet;

fn is_position_within_bounds(pos: (i32, i32), dim: (usize, usize)) -> bool {
    pos.0 >= 0 && pos.0 < (dim.0 as i32) && pos.1 >= 0 && pos.1 < (dim.1 as i32)
}

fn get_adjacent_positions(pos: (i32, i32), dim: (usize, usize)) -> HashSet<(i32, i32)> {
    let mut adj_pos = HashSet::new();

    for (delta_x, delta_y) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        if is_position_within_bounds((pos.0 + delta_x, pos.1 + delta_y), dim) {
            adj_pos.insert((pos.0 + delta_x, pos.1 + delta_y));
        }
    }

    adj_pos
}

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

fn get_neighbour_at_direction(
    pos: (i32, i32),
    dir: Direction,
    plots: &mut Vec<Vec<Plot>>,
    dim: (usize, usize),
) -> Option<Plot> {
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

    let neighbour_pos = (pos.0 + delta_x, pos.1 + delta_y);

    if is_position_within_bounds(neighbour_pos, dim) {
        return Some(plots[neighbour_pos.1 as usize][neighbour_pos.0 as usize]);
    }

    None
}

type Plot = (char, bool);

fn calculate_corners(pos: (i32, i32), dim: (usize, usize), plots: &mut Vec<Vec<Plot>>) -> u32 {
    let n_neighbour = get_neighbour_at_direction(pos, Direction::N, plots, dim);
    let ne_neighbour = get_neighbour_at_direction(pos, Direction::NE, plots, dim);
    let e_neighbour = get_neighbour_at_direction(pos, Direction::E, plots, dim);
    let se_neighbour = get_neighbour_at_direction(pos, Direction::SE, plots, dim);
    let s_neighbour = get_neighbour_at_direction(pos, Direction::S, plots, dim);
    let sw_neighbour = get_neighbour_at_direction(pos, Direction::SW, plots, dim);
    let w_neighbour = get_neighbour_at_direction(pos, Direction::W, plots, dim);
    let nw_neighbour = get_neighbour_at_direction(pos, Direction::NW, plots, dim);

    let current_plot_letter = plots[pos.1 as usize][pos.0 as usize].0;

    let mut corners = 0u32;

    // check for ne inner corner
    if n_neighbour != None
        && e_neighbour != None
        && n_neighbour.unwrap().0 == current_plot_letter
        && e_neighbour.unwrap().0 == current_plot_letter
        && (ne_neighbour == None || ne_neighbour.unwrap().0 != current_plot_letter)
    {
        corners += 1;
    }

    // check for se inner corner
    if s_neighbour != None
        && e_neighbour != None
        && s_neighbour.unwrap().0 == current_plot_letter
        && e_neighbour.unwrap().0 == current_plot_letter
        && (se_neighbour == None || se_neighbour.unwrap().0 != current_plot_letter)
    {
        corners += 1;
    }

    // check for sw inner corner
    if s_neighbour != None
        && w_neighbour != None
        && s_neighbour.unwrap().0 == current_plot_letter
        && w_neighbour.unwrap().0 == current_plot_letter
        && (sw_neighbour == None || sw_neighbour.unwrap().0 != current_plot_letter)
    {
        corners += 1;
    }

    // check for nw inner corner
    if n_neighbour != None
        && w_neighbour != None
        && n_neighbour.unwrap().0 == current_plot_letter
        && w_neighbour.unwrap().0 == current_plot_letter
        && (nw_neighbour == None || nw_neighbour.unwrap().0 != current_plot_letter)
    {
        corners += 1;
    }

    // check for ne outer corner
    if !((n_neighbour != None && (n_neighbour.unwrap().0 == current_plot_letter))
        || (e_neighbour != None && (e_neighbour.unwrap().0 == current_plot_letter)))
    {
        corners += 1;
    }

    // check for sw outer corner
    if !((s_neighbour != None && (s_neighbour.unwrap().0 == current_plot_letter))
        || (w_neighbour != None && (w_neighbour.unwrap().0 == current_plot_letter)))
    {
        corners += 1;
    }

    // check for se outer corner
    if !((s_neighbour != None && (s_neighbour.unwrap().0 == current_plot_letter))
        || (e_neighbour != None && (e_neighbour.unwrap().0 == current_plot_letter)))
    {
        corners += 1;
    }

    // check for nw outer corner
    if !((n_neighbour != None && (n_neighbour.unwrap().0 == current_plot_letter))
        || (w_neighbour != None && (w_neighbour.unwrap().0 == current_plot_letter)))
    {
        corners += 1;
    }

    corners
}

fn explore_region(
    starting_pos: (i32, i32),
    dim: (usize, usize),
    plots: &mut Vec<Vec<Plot>>,
) -> (u32, u32) {
    let mut explored_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut region: HashSet<(i32, i32)> = HashSet::new();
    let mut exploration_queue: Vec<(i32, i32)> = vec![starting_pos];

    while !exploration_queue.is_empty() {
        let current_pos = exploration_queue[0];
        region.insert(current_pos);
        exploration_queue.remove(0);

        if !explored_positions.contains(&current_pos) {
            explored_positions.insert(current_pos);

            plots[current_pos.1 as usize][current_pos.0 as usize].1 = true;

            for adj_pos in get_adjacent_positions(current_pos, dim) {
                if plots[adj_pos.1 as usize][adj_pos.0 as usize].0
                    == plots[current_pos.1 as usize][current_pos.0 as usize].0
                    && plots[adj_pos.1 as usize][adj_pos.0 as usize].1 == false
                {
                    region.insert(adj_pos);
                    exploration_queue.push(adj_pos);
                }
            }
        }
    }

    (
        region.len() as u32,
        region
            .into_iter()
            .fold(0, |sides, plot| sides + calculate_corners(plot, dim, plots)),
    )
}

fn main() {
    let mut plots = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|c| (c, false))
                .collect::<Vec<Plot>>()
        })
        .collect::<Vec<Vec<Plot>>>();

    let dim = (plots[0].len(), plots.len());

    let mut result: u32 = 0;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if !plots[y][x].1 {
                let (area, sides) = explore_region((x as i32, y as i32), dim, &mut plots);
                result += area * sides;
            }
        }
    }

    println!("Result: {result}");
}
