use aoc_2024::*;

#[derive(Debug)]
enum File {
    Block(usize),
    Empty,
}

fn main() {
    let line = &read_file_lines(get_input_file())[0];

    let mut files: Vec<File> = Vec::new();
    let mut number_of_files: usize = 0;
    let mut curr_file_id: usize = 0;

    for i in 0..line.len() {
        if i % 2 == 0 {
            for _ in 0..line.chars().nth(i).unwrap().to_digit(10).unwrap() {
                files.push(File::Block(curr_file_id));
                number_of_files += 1;
            }

            curr_file_id += 1;
        } else {
            for _ in 0..line.chars().nth(i).unwrap().to_digit(10).unwrap() {
                files.push(File::Empty);
            }
        }
    }

    let mut l: usize = 0;
    let mut r: usize = files.len() - 1;
    let mut result: usize = 0;

    while l < number_of_files {
        match files[l] {
            File::Block(file_id) => {
                result += file_id * l;
                l += 1;
            }
            File::Empty => match files[r] {
                File::Empty => {
                    r -= 1;
                }
                File::Block(file_id) => {
                    result += file_id * l;
                    l += 1;
                    r -= 1;
                }
            },
        }
    }

    println!("Result: {result}");
}
