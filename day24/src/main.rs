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

struct BBox {
    minx: i16,
    maxx: i16,
    miny: i16,
    maxy: i16,
}

fn bounding_box(tiles: &HashSet<(i16, i16)>) -> BBox {
    let minx = tiles.iter().min_by(|(_, x1), (_, x2)| x1.cmp(x2)).unwrap().1;
    let maxx = tiles.iter().max_by(|(_, x1), (_, x2)| x1.cmp(x2)).unwrap().1;
    let miny = tiles.iter().min_by(|(y1, _), (y2, _)| y1.cmp(y2)).unwrap().0;
    let maxy = tiles.iter().max_by(|(y1, _), (y2, _)| y1.cmp(y2)).unwrap().0;
    return BBox { minx, maxx, miny, maxy };
}

fn adjacent_tiles(tile: (i16, i16)) -> Vec<(i16, i16)> {
    let (y, x) = tile;
    return vec![
        (y, x+1),
        (y, x-1),
        (y+1, x),
        (y-1, x+1),
        (y-1, x),
        (y+1, x-1),
    ];
}

fn count_adjacent_black_tiles(tiles: &HashSet<(i16, i16)>, tile: (i16, i16)) -> u8 {
    return adjacent_tiles(tile).iter().filter(|t| tiles.contains(t)).collect::<Vec<_>>().len() as u8;
}

fn step(tiles: &HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
    let mut newtiles: HashSet<(i16, i16)> = HashSet::new();
    let bbox = bounding_box(tiles);
    for y in bbox.miny-1..=bbox.maxy+1 {
        for x in bbox.minx-1..=bbox.maxx+1 {
            match (tiles.contains(&(y, x)), count_adjacent_black_tiles(tiles, (y, x))) {
                (true, 0) | (true, 3..=6) => {},
                (true, 1..=2) => {newtiles.insert((y, x));},
                (false, 2) => {newtiles.insert((y, x));},
                (false, 0..=1) | (false, 3..=6) => {},
                _ => panic!("Bad tile count"),
            }
        }
    }
    return newtiles;
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

    for _i in 1..=100 {
        black_tiles = step(&black_tiles);
    }

    println!("{}", black_tiles.len());
}