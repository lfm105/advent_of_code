use aoc_2024::*;

#[derive(Debug, Copy, Clone)]
enum File {
    Block(usize, usize), // id, size, attempted_to_move
    Empty(usize),        // size
}

fn main() {
    let line = &read_file_lines(get_input_file())[0];

    let mut files: Vec<File> = Vec::new();
    let mut curr_file_id: usize = 0;

    for i in 0..line.len() {
        let size = line.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push(File::Block(curr_file_id, size));
            curr_file_id += 1;
        } else {
            files.push(File::Empty(size));
        }
    }

    let mut organised_files: Vec<File> = Vec::new();

    for l in 0..files.len() {
        match files[l] {
            File::Block(_, l_size) => {
                organised_files.push(files[l]);
                files[l] = File::Empty(l_size);
            }
            File::Empty(l_empty_size) => {
                let mut available_size = l_empty_size;
                for r in (l..files.len()).rev() {
                    match files[r] {
                        File::Block(_, r_size) => {
                            if available_size >= r_size {
                                available_size -= r_size;
                                organised_files.push(files[r]);
                                files[r] = File::Empty(r_size);
                            } else {
                            }
                        }
                        File::Empty(_) => {}
                    }
                }
                if available_size > 0 {
                    organised_files.push(File::Empty(available_size));
                }
            }
        }
    }

    let mut result: usize = 0;
    let mut curr_pos: usize = 0;

    for file in organised_files {
        match file {
            File::Block(id, size) => {
                for _ in 0..size {
                    result += curr_pos * id;
                    curr_pos += 1;
                }
            }
            File::Empty(size) => {
                curr_pos += size;
            }
        }
    }

    println!("Result: {result}");
}
