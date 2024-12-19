use aoc_2024::*;

fn main() {
    let lines = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut safe_reports: i32 = 0;

    for line in lines {
        let number_diffs = line
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i32>>();

        safe_reports += (number_diffs
            .iter()
            .all(|number| -3 <= *number && *number <= -1)
            || number_diffs
                .into_iter()
                .all(|number| 1 <= number && number <= 3)) as i32;
    }

    println!("Safe reports: {safe_reports}");
}
