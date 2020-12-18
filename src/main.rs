use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut valid_count = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let limits: Vec<usize> = parts[0].split('-').map(|n| n.parse::<usize>().unwrap()).collect();
        let required_char = parts[1].chars().next().unwrap();
        let test_string = parts[2];

        let count = test_string.chars().filter(|&c| c == required_char).count();
        if count >= limits[0] && count <= limits[1] {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}