use std::{io, io::prelude::*};
use std::{thread};

fn first_along_direction(seatmap: &Vec<Vec<char>>, i: usize, j: usize, dir_update: &fn(i32, i32) -> (i32, i32)) -> char {
    let mut i: i32 = i as i32;
    let mut j: i32 = j as i32;
    loop {
        // Because we don't have destructuring assignment yet.
        let (newi, newj) = dir_update(i, j);
        i = newi;
        j = newj;

        if i < 0 || j < 0 || i >= seatmap.len() as i32 || j >= seatmap[i as usize].len() as i32 {
            // We hit a wall
            break '-';
        }
        match seatmap[i as usize][j as usize] {
            '.' => {},
            'L' => break 'L',
            '#' => break '#',
            _ => panic!("Bad cell in seat map!"),
        }
    }
}

fn count_adj_occupied(seatmap: &Vec<Vec<char>>, i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    let dir_update_functions: Vec<&fn(i32, i32) -> (i32, i32)> = [
        |i, j| (i-1, j-1), |i, j| (i-1, j), |i, j| (i-1, j+1),
        |i, j| (i, j-1), |i, j| (i, j+1),
        |i, j| (i+1, j-1), |i, j| (i+1, j), |i, j| (i+1, j+1),
    ].iter().collect();
    for f in dir_update_functions {
        count += match first_along_direction(seatmap, i, j, f) {
            '-'|'L' => 0,
            '#' => 1,
            _ => panic!("Bad kind of nearest seat along direction"),
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
                    0..=4 => '#',
                    5..=8 => {changed = true; 'L'},
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

        debug_print(&next_map);
        thread::sleep_ms(10);
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