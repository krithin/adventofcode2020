use std::{io, io::prelude::*};
use std::collections::{HashSet, VecDeque};

fn play_round(deck0: &mut VecDeque<u8>, deck1: &mut VecDeque<u8>) {
    println!("{:?}\n{:?}\n", deck0, deck1);
    let card0 = deck0.pop_front().unwrap();
    let card1 = deck1.pop_front().unwrap();

    if card0 as usize > deck0.len() || card1 as usize > deck1.len() {
        if card0 > card1 {
            deck0.push_back(card0);
            deck0.push_back(card1);
        } else {
            deck1.push_back(card1);
            deck1.push_back(card0);
        }
    } else {
        println!("Starting new subgame with subdecks.");
        let mut subdeck0: Vec<u8> = Vec::new();
        let mut subdeck1: Vec<u8> = Vec::new();
        for (i, val) in deck0.iter().enumerate() {
            if i < card0 as usize {
                subdeck0.push(*val);
            } else {
                break;
            }
        }
        for (i, val) in deck1.iter().enumerate() {
            if i < card1 as usize {
                subdeck1.push(*val);
            } else {
                break;
            }
        }
        let (newdeck0, newdeck1) = play_game(&subdeck0, &subdeck1);
        if newdeck1.len() == 0 {
            deck0.push_back(card0);
            deck0.push_back(card1);
        } else if newdeck0.len() == 0 {
            deck1.push_back(card1);
            deck1.push_back(card0);
        }
    }
}

fn play_game(deck0: &[u8], deck1: &[u8]) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut deck0: VecDeque<u8> = deck0.iter().cloned().collect();
    let mut deck1: VecDeque<u8> = deck1.iter().cloned().collect();
    let mut seen_states: HashSet<(Vec<u8>, Vec<u8>)> = HashSet::new();

    loop {
        if seen_states.contains(&(deck0.iter().cloned().collect(), deck1.iter().cloned().collect())) {
            println!("Game over! Player 1 wins because we're in a seen state.");
            return (deck0, VecDeque::new());
        } else if deck0.is_empty() || deck1.is_empty() {
            if deck0.is_empty() { println!("Game over! Player 2 won") } else {println!("Game over! Player 1 won")};
            return (deck0, deck1);
        } else {
            seen_states.insert((deck0.iter().cloned().collect(), deck1.iter().cloned().collect()));
            play_round(&mut deck0, &mut deck1);
            //thread::sleep(time::Duration::from_millis(1));
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut decks = [Vec::<u8>::new(), Vec::<u8>::new()];
    let mut target = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            target = 1;
        } else if line.starts_with("Player ") {
            // pass
        } else {
            decks[target].push(line.parse().unwrap());
        }
    }

    let (final_deck0, final_deck1) = play_game(&decks[0], &decks[1]);
    let final_deck = if final_deck0.len() > final_deck1.len() { final_deck0 } else { final_deck1 };

    let final_score: u32 = final_deck.iter().rev().enumerate().fold(0, |acc, (m, &v)| acc + (m as u32 +1) * v as u32);
    println!("{}", final_score);
}
