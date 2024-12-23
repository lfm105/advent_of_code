use aoc_2024::*;

fn main() {
    let board = read_file_lines(get_input_file())
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let dim = (board[0].len(), board.len());

    let mut x_mas_count = 0;

    for y in 0..dim.1 {
        for x in 0..dim.0 {
            if board[y][x] == 'A' {
                if x >= 1 && y >= 1 && x < (dim.0 - 1) && y < (dim.1 - 1) {
                    let mut m_count = 0;
                    let mut s_count = 0;

                    let letter_at_nw = board[y - 1][x - 1];
                    let letter_at_ne = board[y - 1][x + 1];
                    let letter_at_se = board[y + 1][x + 1];
                    let letter_at_sw = board[y + 1][x - 1];

                    if letter_at_nw == 'M' {
                        m_count += 1;
                    } else if letter_at_nw == 'S' {
                        s_count += 1;
                    }

                    if letter_at_ne == 'M' {
                        m_count += 1;
                    } else if letter_at_ne == 'S' {
                        s_count += 1;
                    }

                    if letter_at_se == 'M' {
                        m_count += 1;
                    } else if letter_at_se == 'S' {
                        s_count += 1;
                    }

                    if letter_at_sw == 'M' {
                        m_count += 1;
                    } else if letter_at_sw == 'S' {
                        s_count += 1;
                    }
                    
                    if m_count == 2 && m_count == s_count
                    {
                        if letter_at_nw == letter_at_ne || letter_at_nw == letter_at_sw || letter_at_ne == letter_at_se || letter_at_se == letter_at_sw
                        {
                            x_mas_count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("X-MAS count: {x_mas_count}");
}
