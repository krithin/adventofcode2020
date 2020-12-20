use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut joltages: Vec<u8> = Vec::new();
    joltages.push(0); // all the original inputs are > 0
    let mut one_count = 0;
    let mut three_count = 0;

    for line in stdin.lock().lines() {
        joltages.push(line.unwrap().parse::<u8>().unwrap());
    }

    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    for i in 1..joltages.len() {
        match joltages[i] - joltages[i-1] {
            1 => one_count = one_count + 1,
            2 => {},
            3 => three_count = three_count + 1,
            _ => panic!("bad joltage"),
        }
    }

    println!("{} {} {}", one_count, three_count, one_count * three_count);
}