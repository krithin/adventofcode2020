use std::{io, io::prelude::*};
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Direction {
    E, W, NW, NE, SW, SE,
}

#[derive(Debug)]
struct DirectionParseError {}

impl std::str::FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Direction, DirectionParseError> {
        match s {
            "e" => Ok(Direction::E),
            "w" => Ok(Direction::W),
            "ne" => Ok(Direction::NE),
            "nw" => Ok(Direction::NW),
            "se" => Ok(Direction::SE),
            "sw" => Ok(Direction::SW),
            _ => Err(DirectionParseError {}),
        }
    }
}

fn flip_tile(tiles: &mut HashSet<(i16, i16)>, directions: &[Direction]) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    for d in directions {
        match d {
            Direction::E => { x += 1 },
            Direction::W => { x -= 1 },
            Direction::NE => { y += 1 },
            Direction::SE => { x += 1; y -= 1 },
            Direction::SW => { y -= 1 },
            Direction::NW => { x -= 1; y += 1},
        }
    }
    if tiles.contains(&(y, x)) {
        tiles.remove(&(y, x));
    } else {
        tiles.insert((y, x));
    }
}

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let mut token = String::new();
    let mut black_tiles: HashSet<(i16, i16)> = HashSet::new();  // NE is +y, E is +x

    for line in input {
        let mut directions: Vec<Direction> = Vec::new();
        for c in line.chars() {
            token += &c.to_string();
            match c {
                'e' | 'w' => {
                    directions.push(token.parse().unwrap());
                    token.clear();
                }
                'n' | 's' => {}
                _ => panic!("bad token"),
            }
        }
        flip_tile(&mut black_tiles, &directions);
    }

    println!("{}", black_tiles.len());
}