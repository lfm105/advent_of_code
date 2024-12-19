use aoc_2024::*;

fn is_report_valid(number_diffs: &Vec<i32>) -> bool {
    number_diffs
        .iter()
        .all(|number| -3 <= *number && *number <= -1)
        || number_diffs
            .iter()
            .all(|number| 1 <= *number && *number <= 3)
}

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
        let number_diffs_without_removal = line
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i32>>();

        let report_safe_without_removal: bool = is_report_valid(&number_diffs_without_removal);

        if report_safe_without_removal {
            safe_reports += 1;
            continue;
        }

        for i in 0..line.len() {
            let mut line_after_removal = line.clone();
            line_after_removal.remove(i);

            let number_diffs_after_removal = line_after_removal
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<i32>>();

            let report_safe_with_change: bool = is_report_valid(&number_diffs_after_removal);

            if report_safe_with_change {
                safe_reports += 1;
                break;
            }
        }
    }

    println!("Safe reports: {safe_reports}");
}
