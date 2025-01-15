use aoc_2024::*;
use regex::Regex;

struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    pub ip: u32,
}

#[derive(Debug)]
enum Operation {
    adv(u32),
    bxl(u32),
    bst(u32),
    jnz(u32),
    bxc,
    out(u32),
    bdv(u32),
    cdv(u32),
}

impl Computer {
    fn get_combo(&self, combo: u32) -> u32 {
        match combo {
            0..=3 => combo,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7.. => panic!(),
        }
    }

    fn do_operation(&mut self, op: Operation) {
        let mut increment_ip: bool = true;

        match op {
            Operation::adv(combo) => self.reg_a = self.reg_a / 2u32.pow(self.get_combo(combo)),
            Operation::bxl(literal) => self.reg_b ^= literal,
            Operation::bst(combo) => self.reg_b = self.get_combo(combo) % 8,
            Operation::jnz(literal) => {
                if self.reg_a != 0 {
                    self.ip = literal;

                    increment_ip = false;
                }
            }
            Operation::bxc => self.reg_b ^= self.reg_c,
            Operation::out(combo) => print!("{},", self.get_combo(combo) % 8),
            Operation::bdv(combo) => self.reg_b = self.reg_a / 2u32.pow(self.get_combo(combo)),
            Operation::cdv(combo) => self.reg_c = self.reg_a / 2u32.pow(self.get_combo(combo)),
        }

        if increment_ip {
            self.ip += 2;
        }
    }
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let reg_a: u32;
    let reg_b: u32;
    let reg_c: u32;

    let register_regex = Regex::new(r"Register [A|B|C]: ([0-9]+)").unwrap();

    let mut input_lines_iter = input_lines.iter();

    if let Some((_, [reg_value])) = register_regex
        .captures(input_lines_iter.next().unwrap())
        .map(|caps| caps.extract())
    {
        reg_a = reg_value.parse::<u32>().unwrap();
    } else {
        panic!();
    }

    if let Some((_, [reg_value])) = register_regex
        .captures(input_lines_iter.next().unwrap())
        .map(|caps| caps.extract())
    {
        reg_b = reg_value.parse::<u32>().unwrap();
    } else {
        panic!();
    }

    if let Some((_, [reg_value])) = register_regex
        .captures(input_lines_iter.next().unwrap())
        .map(|caps| caps.extract())
    {
        reg_c = reg_value.parse::<u32>().unwrap();
    } else {
        panic!();
    }

    input_lines_iter.next();

    let program_regex = Regex::new(r"(\d+)").unwrap();

    let program = program_regex
        .captures_iter(input_lines_iter.next().unwrap())
        .map(|caps| {
            let (_, [number]) = caps.extract();
            number.parse::<u32>().unwrap()
        })
        .collect::<Vec<u32>>();

    let mut computer = Computer {
        reg_a,
        reg_b,
        reg_c,
        ip: 0,
    };

    while computer.ip < program.len() as u32 {
        let operation = match program[computer.ip as usize] {
            4 => Operation::bxc,
            0..=7 => {
                assert!(computer.ip < (program.len() as u32 - 1));

                let operand = program[computer.ip as usize + 1];

                match program[computer.ip as usize] {
                    0 => Operation::adv(operand),
                    1 => Operation::bxl(operand),
                    2 => Operation::bst(operand),
                    3 => Operation::jnz(operand),
                    5 => Operation::out(operand),
                    6 => Operation::bdv(operand),
                    7 => Operation::cdv(operand),
                    _ => panic!(),
                }
            }
            _ => panic!("Invalid opcode! {:?}", program[computer.ip as usize]),
        };

        computer.do_operation(operation);
    }

    println!();
}
