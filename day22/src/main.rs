use std::{io, io::prelude::*};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut decks = [VecDeque::<u8>::new(), VecDeque::<u8>::new()];
    let mut target = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            target = 1;
        } else if line.starts_with("Player ") {
            // pass
        } else {
            decks[target].push_back(line.parse().unwrap());
        }
    }

    // player 1 will win
    while !decks[1].is_empty() {
        let card0 = decks[0].pop_front().unwrap();
        let card1 = decks[1].pop_front().unwrap();
        if card0 > card1 {
            decks[0].push_back(card0);
            decks[0].push_back(card1);
        } else {
            decks[1].push_back(card1);
            decks[1].push_back(card0);
        }
        println!("{:?}\n{:?}", decks[0], decks[1]);
    }

    let final_score: u32 = decks[0].iter().rev().enumerate().fold(0, |acc, (m, &v)| acc + (m as u32 +1) * v as u32);
    println!("{}", final_score);
}
