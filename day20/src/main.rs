use std::{io, io::prelude::*};
use std::collections::HashSet;
use std::collections::VecDeque;

// reverses the bits of a 10-bit int.
fn rev10(mut n: u16) -> u16 {
    let mut result = 0;
    for _i in 1..=10 {
        result = (result << 1) + n % 2;
        n = n >> 1;
    }
    assert_eq!(0, n);
    return result;
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    // order: top, right, down, left. Each side is read left to right or top to bottom.
    id: u16,
    sides: [u16; 4],
}

impl Tile {
    fn rotate(&self) -> Tile {
        let [top, right, down, left] = self.sides;
        return Tile { id: self.id, sides: [rev10(left), top, rev10(right), down] };
    }

    fn flip(&self) -> Tile {
        let [top, right, down, left] = self.sides;
        return Tile { id: self.id, sides: [rev10(top), left, rev10(down), right] };
    }

    fn all_orientations(&self) -> [Tile; 8] {
        let flip = self.flip();
        return [
            *self,
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            flip,
            flip.rotate(),
            flip.rotate().rotate(),
            flip.rotate().rotate().rotate(),
        ];
    }

    // returns true if self can appear to the right of other
    fn match_left(&self, other: &Tile) -> bool {
        return self.sides[3] == other.sides[1];
    }

    fn match_top(&self, other: &Tile) -> bool {
        return self.sides[0] == other.sides[2];
    }
}

fn parse10(chars: &[char]) -> u16 {
    let mut result: u16 = 0;
    for c in chars {
        result = (result << 1) +
            match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Bad input char!"),
            };
    }
    assert_eq!(result, result & 0b1111111111);
    return result;
}

fn parse_tile(tile_id: u16, tile_lines: &[String]) -> Tile {
    let top = parse10(&tile_lines[0].chars().collect::<Vec<_>>());
    let right = parse10(&tile_lines.iter().map(|s| s.chars().last().unwrap()).collect::<Vec<_>>());
    let down = parse10(&tile_lines.iter().last().unwrap().chars().collect::<Vec<_>>());
    let left = parse10(&tile_lines.iter().map(|s| s.chars().next().unwrap()).collect::<Vec<_>>());

    return Tile { id: tile_id, sides: [top, right, down, left] };
}

#[derive(Clone, Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    tile_ids: HashSet<u16>,
    side_len: usize,
}

impl Board {
    fn new(side_len: usize) -> Board {
        let mut b = Board { tiles: Vec::with_capacity(side_len), tile_ids: HashSet::new(), side_len };
        for _ in 1..=side_len {
            b.tiles.push(Vec::new());
        }
        return b;
    }

    fn push(&mut self, tile: Tile) {
        for row in &mut self.tiles {
            if row.len() < self.side_len {
                assert_eq!(true, self.tile_ids.insert(tile.id));
                row.push(tile);
                return;
            }
        }
        panic!("Can't push, board is full!");
    }

    // returns tiles to the left, top
    fn required_matches(&self) -> (Option<&Tile>, Option<&Tile>) {
        if self.tile_ids.is_empty() { 
            return (None, None); 
        } else if self.tiles[0].len() < self.side_len {
            return (Some(self.tiles[0].last().unwrap()), None);
        } else {
            for i in 1..=self.side_len {
                if self.tiles[i].len() < self.side_len {
                    if self.tiles[i].is_empty() {
                        return (None, Some(&self.tiles[i-1][0]));
                    } else {
                        return (Some(self.tiles[i].last().unwrap()), Some(&self.tiles[i-1][self.tiles[i].len()]));
                    }
                }
            }
        }
        panic!("Board is full!");
    }
}

fn search(tiles: &[Tile], side_len: usize) -> Board {
    let mut queue = VecDeque::<Board>::new();

    // seed the queue
    for tile in tiles {
        for tile_orientation in tile.all_orientations().iter() {
            let mut b = Board::new(side_len);
            b.push(*tile_orientation);
            queue.push_back(b);
        }
    }

    while !queue.is_empty() {
        let b = queue.pop_back().unwrap();
        //println!("{} {}", queue.len(), b.tile_ids.len());
        if b.tile_ids.len() == tiles.len() {
            return b;
        }
        let (left_tile, top_tile) = b.required_matches();
        for tile in tiles {
            if b.tile_ids.contains(&tile.id) { continue; }
            for tile_orientation in tile.all_orientations().iter() {
                match (left_tile, top_tile) {
                    (Some(left_tile), Some(top_tile)) => {
                        if tile_orientation.match_left(left_tile) && tile_orientation.match_top(top_tile) {
                            let mut newb = b.clone();
                            newb.push(*tile_orientation);
                            queue.push_back(newb);
                        }
                    },
                    (None, Some(top_tile)) => {
                        if tile_orientation.match_top(top_tile) {
                            let mut newb = b.clone();
                            newb.push(*tile_orientation);
                            queue.push_back(newb);
                        }
                    },
                    (Some(left_tile), None) => {
                        if tile_orientation.match_left(left_tile) {
                            let mut newb = b.clone();
                            newb.push(*tile_orientation);
                            queue.push_back(newb);
                        }
                    }
                    (None, None) => panic!("Expected board to be nonempty!"),
                }
            }
        }
    }

    panic!("Couldn't solve the puzzle!");
}

fn main() {
    let stdin = io::stdin();
    let mut tile_num: u16 = 0;
    let mut tile_lines: Vec<String> = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            tiles.push(parse_tile(tile_num, &tile_lines));
            tile_lines.clear();
        } else if line.starts_with("Tile ") {
            tile_num = line[5..=8].parse().unwrap();
            continue;
        } else {
            tile_lines.push(line);
        }
    }
    let side_len = (tiles.len() as f64).sqrt() as usize;
    let part1_finished_board = search(&tiles, side_len);
    
    let t = part1_finished_board.tiles;
    let corner_mult = [t[0][0], t[0][side_len-1], t[side_len-1][0], t[side_len-1][side_len-1]]
        .iter().map(|t| t.id).fold(1 as u64, |acc, val| acc * val as u64);
    println!("{}", corner_mult);
}
