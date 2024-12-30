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

type Plot = (char, bool);

fn explore_region(
    starting_pos: (i32, i32),
    dim: (usize, usize),
    plots: &mut Vec<Vec<Plot>>,
) -> (u32, u32) {
    let (mut area, mut perimeter): (u32, u32) = (1, 4);

    plots[starting_pos.1 as usize][starting_pos.0 as usize].1 = true;

    for adj_pos in get_adjacent_positions(starting_pos, dim) {
        if plots[adj_pos.1 as usize][adj_pos.0 as usize].0
            == plots[starting_pos.1 as usize][starting_pos.0 as usize].0
        {
            if !plots[adj_pos.1 as usize][adj_pos.0 as usize].1 {
                let (adj_area, adj_perimeter) = explore_region(adj_pos, dim, plots);
                area += adj_area;
                perimeter += adj_perimeter;
            }
            perimeter -= 1;
        }
    }

    (area, perimeter)
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
                let (area, perimeter) = explore_region((x as i32, y as i32), dim, &mut plots);
                result += area * perimeter;
            }
        }
    }

    println!("Result: {result}");
}
