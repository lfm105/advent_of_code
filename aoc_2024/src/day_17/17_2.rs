use aoc_2024::*;
use regex::Regex;

fn find(program: &[u8], ans: u64) -> Option<u64> {
    if program == [] {
        return Some(ans);
    } else {
        for i in 0..8 {
            let a: u64 = (ans << 3) + i;
            let mut b = a % 8;
            b ^= 1;
            let c = a >> b;
            b ^= c;
            b ^= 4;
            if b % 8 == program[program.len() - 1] as u64 {
                let sub = find(&program[0..program.len() - 1], a);
                if sub == None {
                    continue;
                }
                return sub;
            }
        }

        None
    }
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let program_regex = Regex::new(r"(\d+)").unwrap();

    let program = program_regex
        .captures_iter(input_lines.iter().skip(4).next().unwrap())
        .map(|caps| {
            let (_, [number]) = caps.extract();
            number.parse::<u8>().unwrap()
        })
        .collect::<Vec<u8>>();

    dbg!(find(&program, 0));
}
