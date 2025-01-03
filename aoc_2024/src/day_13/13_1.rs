use aoc_2024::*;
use regex::Regex;
use std::error::Error;

type Machine = (i32, i32, i32, i32, i32, i32); // A_X, A_Y, B_X, B_Y, X, Y

fn main() -> Result<(), Box<dyn Error>> {
    let input_lines = read_file_lines(get_input_file());

    let button_regex = Regex::new(r"Button [A|B]: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let target_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let machines = input_lines
        .iter()
        .step_by(4)
        .zip(
            input_lines
                .iter()
                .skip(1)
                .step_by(4)
                .zip(input_lines.iter().skip(2).step_by(4)),
        )
        .map(|(a, (b, c))| {
            let mut machine: Machine = (0, 0, 0, 0, 0, 0);

            for (_, [x, y]) in button_regex.captures_iter(a).map(|cc| cc.extract()) {
                machine.0 = x.parse::<i32>().unwrap();
                machine.1 = y.parse::<i32>().unwrap();
            }
            for (_, [x, y]) in button_regex.captures_iter(b).map(|cc| cc.extract()) {
                machine.2 = x.parse::<i32>().unwrap();
                machine.3 = y.parse::<i32>().unwrap();
            }
            for (_, [x, y]) in target_regex.captures_iter(c).map(|cc| cc.extract()) {
                machine.4 = x.parse::<i32>().unwrap();
                machine.5 = y.parse::<i32>().unwrap();
            }

            machine
        })
        .collect::<Vec<Machine>>();

    dbg!(&machines);

    let mut tokens: u64 = 0;

    machines.into_iter().for_each(|machine| {
        let b_pushes: f32 = (machine.0 * machine.5 - machine.1 * machine.4) as f32
            / (machine.0 * machine.3 - machine.1 * machine.2) as f32;

        let a_pushes: f32 = (machine.4 - machine.2 * b_pushes as i32) as f32 / machine.0 as f32;

        if a_pushes.fract() <= 0.000001
            && b_pushes.fract() <= 0.000001
            && a_pushes <= 100f32
            && b_pushes <= 100f32
        {
            tokens += 3 * a_pushes as u64;
            tokens += b_pushes as u64;
        }
    });

    println!("Tokens: {tokens}");

    Ok(())
}
