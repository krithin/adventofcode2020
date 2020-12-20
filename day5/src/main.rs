use std::{io, io::prelude::*};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut missing_seatnums = HashSet::<u16>::new();
    for i in 0..976 {
        missing_seatnums.insert(i);
    }

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut seatnum: u16 = 0;

        for c in line[..7].chars() {
            match c {
                'B' => { seatnum = seatnum * 2 + 1; }
                'F' => { seatnum = seatnum * 2 + 0; }
                _ => { panic!("Bad input!"); }
            }
        }

        for c in line[7..].chars() {
            match c {
                'R' => { seatnum = seatnum * 2 + 1; }
                'L' => { seatnum = seatnum * 2 + 0; }
                _ => { panic!("Bad input!"); }
            }
        }

        missing_seatnums.remove(&seatnum);
    }

    println!("{:?}", missing_seatnums.iter().collect::<Vec<&u16>>());
}