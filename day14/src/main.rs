use std::{io, io::prelude::*};
use std::collections::HashMap;

fn combine(mask: &str, addr: u64) -> String {
    assert_eq!(mask.len(), 36);
    let addr = format!("{:0>36b}", addr);
    return mask.chars()
            .zip(addr.chars())
            .map(
                |(m, v)| match (m, v) {
                    ('0', '0') => '0',
                    ('0', '1') | ('1', '0') | ('1', '1') => '1',
                    ('X', '0') | ('X', '1') => 'X',
                    _ => panic!("bad m,v"),
                }
            ).collect();
}

fn generate_all_addresses(combined_addr: &str) -> Vec<u64> {
    let mut output: Vec<Vec<u8>> = Vec::new();
    output.push(Vec::new());

    for c in combined_addr.chars() {
        match c {
            '0' => { for s in output.iter_mut() { s.push(0) } },
            '1' => { for s in output.iter_mut() { s.push(1) } },
            'X' => { 
                let mut copy = output.clone();
                for s in output.iter_mut() {
                    s.push(0);
                }
                for s in copy.iter_mut() {
                    s.push(1);
                }
                output.append(&mut copy);
            }
            _ => panic!("Bad c"),
        }
    }

    return output
        .iter()
        .map(
            |v| v
                .iter()
                .fold(0 as u64, |acc, &x| acc * 2 + x as u64)
        ).collect();
}

fn main() {
    let stdin = io::stdin();

    let mut mask: String = "".to_string();
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        match &line[0..4] {
            "mask" => {
                mask = line.split_whitespace().collect::<Vec<&str>>()[2].to_string();
            },
            "mem[" => {
                let mut line_iter = line.split_whitespace();
                let dst: &str = line_iter.next().unwrap();
                let dst: u64 = dst[4..dst.len()-1].parse().unwrap();
                assert_eq!(Some("="), line_iter.next());
                let val: u64 = line_iter.next().unwrap().parse().unwrap();

                let targets: Vec<u64> = generate_all_addresses(&combine(&mask, dst));
                for target in targets {
                    mem.insert(target, val);
                }
            },
            _ => panic!("Bad input line"),
        }
    }

    let sum: u64 = mem.iter().map(|(_k, v)| v).sum();
    println!("{}", sum);
}
