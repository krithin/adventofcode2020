use std::{io, io::prelude::*};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut current_group_answers: Vec<HashSet::<char>> = Vec::<HashSet::<char>>::new();
    let mut total_answers = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.as_str() {
            "" => {
                let current_group_intersection = 
                    current_group_answers
                        .iter()
                        .fold(
                            current_group_answers[0].iter().cloned().collect::<HashSet::<char>>(),
                            |acc, line| acc.intersection(line).cloned().collect()
                        );
                total_answers = total_answers + current_group_intersection.len();
                current_group_answers.clear();
            },
            _ => {
                current_group_answers.push(line.chars().collect());
            }
        }
    }

    println!("{}", total_answers);
}