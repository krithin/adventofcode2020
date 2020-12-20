use std::{io, io::prelude::*};
use std::{thread};

fn count_adj_occupied(seatmap: &Vec<Vec<char>>, i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    if i > 0 {
        if j > 0 {
            count += match seatmap[i-1][j-1] { '#' => 1, _ => 0 };
        }
        count += match seatmap[i-1][j] { '#' => 1, _ => 0 };
        if j < seatmap[i].len() - 1 {
            count += match seatmap[i-1][j+1] { '#' => 1, _ => 0 };
        }
    }
    if j > 0 {
        count += match seatmap[i][j-1] { '#' => 1, _ => 0 };
    }
    if j < seatmap[i].len() - 1 {
        count += match seatmap[i][j+1] { '#' => 1, _ => 0 };
    }
    if i < seatmap.len() - 1 {
        if j > 0 {
            count += match seatmap[i+1][j-1] { '#' => 1, _ => 0 };
        }
        count += match seatmap[i+1][j] { '#' => 1, _ => 0 };
        if j < seatmap[i].len() - 1 {
            count += match seatmap[i+1][j+1] { '#' => 1, _ => 0 };
        }
    }
    return count;
}

fn tick(seatmap: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut next_map = seatmap.clone();
    let mut changed = false;
    for i in 0..next_map.len() {
        for j in 0..next_map[i].len() {
            next_map[i][j] = match seatmap[i][j] {
                '.' => '.',
                'L' => match count_adj_occupied(seatmap, i, j) {
                    0 => {changed = true; '#'},
                    1..=8 => 'L',
                    _ => panic!("unexpected adj count"),
                },
                '#' => match count_adj_occupied(seatmap, i, j) {
                    0..=3 => '#',
                    4..=8 => {changed = true; 'L'},
                    _ => panic!("unexpected adj count"),
                },
                _ => panic!("Unexpected char in seat map"),
            }
        }
    }
    return (next_map, changed);
}

fn debug_print(seatmap: &Vec<Vec<char>>) {
    for row in seatmap {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn main() {
    let stdin = io::stdin();
    let mut seatmap: Vec<Vec<char>> = Vec::with_capacity(91);

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        seatmap.push(line.chars().collect());
    }

    let final_map = loop {
        let (next_map, changed) = tick(&seatmap);

        //debug_print(&next_map);
        //thread::sleep_ms(10);
        if !changed { break next_map };
        seatmap = next_map;
    };

    let final_occupied_count: u32 =
        final_map
            .iter()
            .map(
                |row|
                row.iter().map(
                    |c|
                    match c { '#' => 1, '.'|'L' => 0, _ => panic!("Bad cell")}
                )
                .sum::<u32>()
            )
            .sum();
    println!("{}", final_occupied_count);
}