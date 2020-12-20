use std::{io, io::prelude::*};

fn main() {
    let stdin = io::stdin();
    let target: u64 = 1038347917;
    let mut sums: Vec<u64> = Vec::new();
    let mut values: Vec<u64> = Vec::new();

    for line in stdin.lock().lines() {
        let n = line.unwrap().parse::<u64>().unwrap();
        values.push(n);
        sums.push(0);
        for i in 1..sums.len() {
            sums[i] += n;
            if sums[i] == target {
                println!("i: {}, len: {}", i, sums.len());
                let mut min = values[i];
                let mut max = values[i];
                for j in values[i..].iter() {
                    if j < &min {
                        min = *j;
                    }
                    if j > &max {
                        max = *j;
                    }
                }
                println!("sum {}", min+max);
            }
        }
    }
}