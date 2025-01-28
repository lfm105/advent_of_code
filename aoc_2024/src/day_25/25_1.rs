use aoc_2024::*;
use std::collections::HashSet;

type Lock = [u32; 5];
type Key = Lock;

fn lock_key_fit(lock: &Lock, key: &Key) -> bool {
    (lock[0] + key[0] <= 5)
        && (lock[1] + key[1] <= 5)
        && (lock[2] + key[2] <= 5)
        && (lock[3] + key[3] <= 5)
        && (lock[4] + key[4] <= 5)
}

fn main() {
    let input_lines = read_file_lines(get_input_file());

    let mut locks: HashSet<Lock> = HashSet::new();
    let mut keys: HashSet<Key> = HashSet::new();

    for i in 0..=(input_lines.len() / 8) {
        let top = &input_lines[i * 8];

        if top.contains("#") {
            let mut lock = [0u32; 5];

            for j in 0..7usize {
                for k in 0..5usize {
                    if input_lines[i * 8 + j].chars().nth(k) == Some('#') {
                        lock[k] = j as u32;
                    }
                }
            }

            locks.insert(lock);
        } else {
            let mut key = [0u32; 5];

            for j in 0..7usize {
                for k in 0..5usize {
                    if input_lines[i * 8 + 7 - j - 1].chars().nth(k) == Some('#') {
                        key[k] = j as u32;
                    }
                }
            }

            keys.insert(key);
        }
    }

    let mut fitting_pairs = 0u32;

    locks.iter().for_each(|lock| {
        keys.iter().for_each(|key| {
            if lock_key_fit(lock, key) {
                fitting_pairs += 1;
            }
        });
    });

    println!("There are {fitting_pairs} lock/key pairs which fit together");
}
