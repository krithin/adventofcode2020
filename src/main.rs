use std::{io, io::prelude::*};
use std::collections::HashSet;
use std::collections::VecDeque;

fn generate_all_sums(preamble: &VecDeque<u64>) -> HashSet<u64> {
    let mut sums: HashSet<u64> = HashSet::with_capacity(300);
    for i in 0..preamble.len() {
        for j in i+1..preamble.len() {
            sums.insert(preamble[i] + preamble[j]);
        }
    }
    return sums;
}

fn main() {
    let stdin = io::stdin();
    let mut preamble: VecDeque<u64> = VecDeque::with_capacity(25);

    for line in stdin.lock().lines() {
        let n = line.unwrap().parse::<u64>().unwrap();
        if preamble.len() < 25 {
            preamble.push_back(n)
        } else {
            if !generate_all_sums(&preamble).contains(&n) {
                println!("number not found: {}", n);
            } else {
                preamble.pop_front();
                preamble.push_back(n);
            }
        }
    }
}