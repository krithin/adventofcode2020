use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let input: Vec::<u32> =
        stdin
            .lock()
            .lines()
            .map(|l| l.unwrap().parse::<u32>().unwrap())
            .collect();

    for i in 0..input.len() {
        for j in i+1..input.len() {
            for k in j+1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("{}", input[i] * input[j] * input[k]);
                }
            }
        }
    }
}
