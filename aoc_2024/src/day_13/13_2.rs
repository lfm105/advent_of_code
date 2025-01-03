use aoc_2024::*;
use regex::Regex;
use std::error::Error;

type Machine = (i64, i64, i64, i64, i64, i64); // A_X, A_Y, B_X, B_Y, X, Y

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
                machine.0 = x.parse::<i64>().unwrap();
                machine.1 = y.parse::<i64>().unwrap();
            }
            for (_, [x, y]) in button_regex.captures_iter(b).map(|cc| cc.extract()) {
                machine.2 = x.parse::<i64>().unwrap();
                machine.3 = y.parse::<i64>().unwrap();
            }
            for (_, [x, y]) in target_regex.captures_iter(c).map(|cc| cc.extract()) {
                machine.4 = 10000000000000 + x.parse::<i64>().unwrap();
                machine.5 = 10000000000000 + y.parse::<i64>().unwrap();
            }

            machine
        })
        .collect::<Vec<Machine>>();

    dbg!(&machines);

    let mut tokens: u64 = 0;

    machines.into_iter().for_each(|machine| {
        let b_pushes: f64 = (machine.0 * machine.5 - machine.1 * machine.4) as f64
            / (machine.0 * machine.3 - machine.1 * machine.2) as f64;

        let a_pushes: f64 = (machine.4 - machine.2 * b_pushes as i64) as f64 / machine.0 as f64;

        if a_pushes.fract() <= 0.000001 && b_pushes.fract() <= 0.000001 {
            tokens += 3 * a_pushes as u64;
            tokens += b_pushes as u64;
        }
    });

    println!("Tokens: {tokens}");

    Ok(())
}
