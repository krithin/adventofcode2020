use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut valid_count = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let required_positions: Vec<usize> = parts[0].split('-').map(|n| n.parse::<usize>().unwrap()).collect();
        let required_char = parts[1].chars().next().unwrap();

        let mut test_chars = parts[2].chars();
        let first_char = test_chars.nth(required_positions[0] - 1).unwrap();
        let second_char = test_chars.nth(required_positions[1] - required_positions[0] - 1).unwrap();

        if (first_char == required_char) ^ (second_char == required_char) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}