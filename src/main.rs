use std::{io, io::prelude::*};
use std::collections::HashMap;

fn count_paths(starting_joltage: u8, adapters: &[u8], memo: &mut HashMap<(u8, usize), u64>) -> u64 {
    if adapters[0] - starting_joltage > 3 {
        memo.insert((0, adapters.len()), 0);
        return 0;
    } else if adapters.len() == 1 {
        memo.insert((starting_joltage, 1), 1);
        return 1;
    } else if memo.contains_key(&(starting_joltage, adapters.len())) {
        return memo.get(&(starting_joltage, adapters.len())).unwrap().clone();
    } else {
        let c = count_paths(starting_joltage, &adapters[1..], memo) + count_paths(adapters[0], &adapters[1..], memo);
        memo.insert((starting_joltage, adapters.len()), c);
        return c;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut joltages: Vec<u8> = Vec::new();
    //joltages.push(0); // all the original inputs are > 0
    let mut memo: HashMap<(u8, usize), u64> = HashMap::new();

    for line in stdin.lock().lines() {
        joltages.push(line.unwrap().parse::<u8>().unwrap());
    }

    joltages.sort();
    joltages.push(joltages.last().unwrap() + 3);

    println!("{}", count_paths(0, &joltages, &mut memo));
}